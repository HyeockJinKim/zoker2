use crypto::digest::Digest;
use crypto::sha2::Sha256;
use hex::FromHex;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;
use crate::zkboo::utils::convert_u32_to_u8;

macro_rules! get_bit {
    ($x: expr, $i: expr) => {{
        ($x >> $i) & 0x01
    }};
}

macro_rules! set_bit {
    ($x: expr, $i: expr, $b: expr) => {{
        if ($b & 1) != 0 {
            $x | ((1 as u32) << $i)
        } else {
            $x & (!((1 as u32) << $i))
        }
    }};
}

pub type IKosResult<T> = Result<T, IKosError>;

#[derive(Debug, PartialEq)]
pub struct IKosError {
    pub error: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IKosView {
    pub rand_tape_seed: Vec<u8>,
    pub in_data: Vec<u32>,
    pub out_data: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IKosContext {
    pub ikos_view: IKosView,
    randomness: Vec<u32>,
    pub used_rand_ctr: usize,
    pub out_view_ctr: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IKosVariable4P {
    pub value: Vec<u32>,
    ctx: Rc<RefCell<Vec<IKosContext>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IKosVariable4V {
    pub value: Vec<u32>,
    pub ctx: Rc<RefCell<Vec<IKosContext>>>,
}

fn generate_random(num: usize) -> Vec<u8> {
    let mut randoms = vec![];
    let mut rng = rand::thread_rng();
    for _ in 0..num {
        randoms.push(rng.gen());
    }
    randoms
}

fn generate_randomness(key: &[u8]) -> Vec<u32> {
    let mut randoms = vec![];
    let mut sha = Sha256::new();
    sha.input(key);
    let hash = <[u8; 32]>::from_hex(sha.result_str()).unwrap();
    for i in 0..8 {
        let random = (hash[4 * i] as u32) << 24
            | (hash[4 * i + 1] as u32) << 16
            | (hash[4 * i + 2] as u32) << 8
            | (hash[4 * i + 3] as u32);
        randoms.push(random);
    }
    randoms
}

pub fn get_next_random_from_context(ctx: &mut IKosContext) -> IKosResult<u32> {
    if ctx.randomness.len() <= ctx.used_rand_ctr {
        ctx.randomness = generate_randomness(convert_u32_to_u8(ctx.randomness.as_ref()).as_ref());
        ctx.used_rand_ctr = 0;
    }
    let rand = ctx.randomness[ctx.used_rand_ctr as usize];
    ctx.used_rand_ctr += 1;
    Ok(rand)
}

impl Default for IKosView {
    fn default() -> Self {
        Self::new()
    }
}

impl IKosView {
    pub fn new() -> Self {
        let ozkb_rand_tape_seed_len = 16;
        IKosView {
            rand_tape_seed: generate_random(ozkb_rand_tape_seed_len),
            in_data: vec![],
            out_data: vec![],
        }
    }
}

impl IKosContext {
    pub fn new() -> Self {
        let ikos_view = IKosView::new();
        let seed = ikos_view.rand_tape_seed.clone();
        IKosContext {
            ikos_view,
            randomness: generate_randomness(&seed),
            used_rand_ctr: 0,
            out_view_ctr: 0,
        }
    }

    pub fn new_views(ikos_view: IKosView) -> Self {
        let seed = ikos_view.rand_tape_seed.clone();
        IKosContext {
            ikos_view,
            randomness: generate_randomness(&seed),
            used_rand_ctr: 0,
            out_view_ctr: 0,
        }
    }

    pub fn commit_ikos_context(&mut self) -> [u8; 32] {
        let mut sha = Sha256::new();
        let mut vec = self.ikos_view.rand_tape_seed.clone();
        vec.extend(convert_u32_to_u8(&*self.ikos_view.out_data.clone()));
        sha.input(&vec);
        let res = <[u8; 32]>::from_hex(sha.result_str()).unwrap();
        res
    }
}

impl IKosVariable4P {
    pub fn new_value(value: u32) -> Self {
        IKosVariable4P {
            value: vec![value; 3],
            ctx: Rc::new(RefCell::new(vec![])),
        }
    }

    pub fn new_share(value: Vec<u32>, ctx: Rc<RefCell<Vec<IKosContext>>>) -> Self {
        IKosVariable4P { value, ctx }
    }

    fn is_empty_context(&self) -> bool {
        self.ctx.borrow().is_empty()
    }

    pub fn negate(mut self) -> Self {
        for i in 0..3 {
            self.value[i] = !self.value[i];
        }
        self
    }

    pub fn xor(mut self, rhs: &IKosVariable4P) -> Self {
        for i in 0..3 {
            self.value[i] ^= rhs.value[i];
        }
        self
    }

    pub fn rshift(mut self, n: u32) -> Self {
        for i in 0..3 {
            self.value[i] >>= n;
        }
        self
    }

    pub fn lshift(mut self, n: u32) -> Self {
        for i in 0..3 {
            self.value[i] <<= n;
        }
        self
    }

    pub fn bit_and(mut self, rhs: &IKosVariable4P) -> Self {
        let mut rand = vec![0; 3];
        let mut out = vec![0; 3];

        if self.is_empty_context() && rhs.is_empty_context() {
            for i in 0..3 {
                self.value[i] &= rhs.value[i];
            }
            return self;
        }

        if self.is_empty_context() {
            return rhs.clone().bit_and(&self);
        }

        for (i, random) in rand.iter_mut().enumerate().take(3) {
            *random = get_next_random_from_context(&mut self.ctx.borrow_mut()[i]).unwrap();
        }
        for i in 0..3 {
            out[i] = (self.value[i] & rhs.value[(i + 1) % 3])
                ^ (self.value[(i + 1) % 3] & rhs.value[i])
                ^ (self.value[i] & rhs.value[i])
                ^ rand[i]
                ^ rand[(i + 1) % 3];
        }

        for (i, &out_value) in out.iter().enumerate().take(3) {
            self.value[i] = out_value;
            self.ctx.borrow_mut()[i]
                .ikos_view
                .out_data
                .push(self.value[i]);
        }
        self
    }

    pub fn bit_or(mut self, rhs: &IKosVariable4P) -> Self {
        for i in 0..3 {
            self.value[i] |= rhs.value[i];
        }
        self
    }

    pub fn gt(mut self, rhs: &IKosVariable4P) -> Self {
        let minus_rhs = rhs.clone().negate().add_op(&IKosVariable4P::new_value(1));
        let mut minus_gt = self.add_op(&minus_rhs);
        for i in 0..3 {
            let val = minus_gt.value[i] as u32;
            let lsb = val >> 31;
            if lsb == 0 {
                minus_gt.value[i] = u32::MAX;
            } else {
                minus_gt.value[i] = 0;
            }
        }
        minus_gt
    }

    pub fn if_op(mut self, rhs: &IKosVariable4P) -> Self {
        self.bit_and(rhs)
    }

    pub fn add_op(mut self, rhs: &IKosVariable4P) -> Self {
        let mut a = vec![0; 3];
        let mut b = vec![0; 3];
        let mut rand = vec![0; 3];
        let mut out = vec![0; 3];

        if self.is_empty_context() && rhs.is_empty_context() {
            // constant calculation
            for i in 0..3 {
                self.value[i] += rhs.value[i];
            }
            return self;
        }

        if self.is_empty_context() {
            return rhs.clone().add_op(&self);
        }
        for (i, random) in rand.iter_mut().enumerate().take(3) {
            *random = get_next_random_from_context(&mut self.ctx.borrow_mut()[i]).unwrap();
        }

        for i in 0..31 {
            for j in 0..3 {
                a[j] = get_bit!(self.value[j] ^ out[j], i);
                b[j] = get_bit!(rhs.value[j] ^ out[j], i);
            }
            for j in 0..3 {
                let c = (a[j] & b[(j + 1) % 3])
                    ^ (a[(j + 1) % 3] & b[j])
                    ^ get_bit!(rand[(j + 1) % 3], i);
                out[j] = set_bit!(
                    out[j],
                    i + 1,
                    (c ^ (a[j] & b[j]) ^ (get_bit!(out[j], i)) ^ (get_bit!(rand[j], i)))
                );
            }
        }

        for (i, &out_value) in out.iter().enumerate().take(3) {
            self.value[i] = self.value[i] ^ rhs.value[i] ^ out_value;
            self.ctx.borrow_mut()[i].ikos_view.out_data.push(out_value);
        }
        self
    }
}

impl IKosVariable4V {
    pub fn new_value(value: u32) -> Self {
        IKosVariable4V {
            value: vec![value; 2],
            ctx: Rc::new(RefCell::new(vec![])),
        }
    }

    pub fn new_share(value: Vec<u32>, ctx: Rc<RefCell<Vec<IKosContext>>>) -> Self {
        IKosVariable4V { value, ctx }
    }

    // TODO: 이 함수들은 P 에서도 사용함
    fn is_empty_context(&self) -> bool {
        self.ctx.borrow().is_empty()
    }

    pub fn negate(mut self) -> Self {
        for i in 0..2 {
            self.value[i] = !self.value[i];
        }
        self
    }

    pub fn xor(mut self, rhs: &IKosVariable4V) -> Self {
        for i in 0..2 {
            self.value[i] ^= rhs.value[i];
        }
        self
    }

    pub fn rshift(mut self, n: u32) -> Self {
        for i in 0..2 {
            self.value[i] >>= n;
        }
        self
    }

    pub fn lshift(mut self, n: u32) -> Self {
        for i in 0..2 {
            self.value[i] <<= n;
        }
        self
    }

    pub fn require_reconstruct(ctx: &[IKosContext]) -> bool {
        ctx[0].ikos_view.out_data.len() != ctx[1].ikos_view.out_data.len()
    }

    pub fn bit_and(mut self, rhs: &IKosVariable4V) -> IKosResult<Self> {
        let mut rand = vec![0; 2];
        if self.is_empty_context() && rhs.is_empty_context() {
            for i in 0..2 {
                self.value[i] &= rhs.value[i];
            }
            return Ok(self);
        }

        if self.is_empty_context() {
            return rhs.clone().bit_and(&self);
        }

        for (i, random) in rand.iter_mut().enumerate().take(2) {
            *random = get_next_random_from_context(&mut self.ctx.borrow_mut()[i]).unwrap();
        }

        let out = (self.value[0] & rhs.value[1])
            ^ (self.value[1] & rhs.value[0])
            ^ (self.value[0] & rhs.value[0])
            ^ rand[0]
            ^ rand[1];

        if !IKosVariable4V::require_reconstruct(&self.ctx.borrow()) {
            if out != self.ctx.borrow()[0].ikos_view.out_data[self.ctx.borrow()[0].out_view_ctr] {
                return Err(IKosError {
                    error: String::from("_IkosVariable4V & operation fail."),
                });
            }
        } else {
            self.ctx.borrow_mut()[0].ikos_view.out_data.push(out);
        }
        self.value[0] = out;
        self.value[1] = self.ctx.borrow()[1].ikos_view.out_data[self.ctx.borrow()[1].out_view_ctr];
        for i in 0..2 {
            self.ctx.borrow_mut()[i].out_view_ctr += 1;
        }

        Ok(self)
    }

    pub fn bit_or(mut self, rhs: &IKosVariable4V) -> Self {
        for i in 0..2 {
            self.value[i] |= rhs.value[i];
        }
        self
    }

    pub fn add_op(mut self, rhs: &IKosVariable4V) -> IKosResult<Self> {
        let mut a = vec![0; 2];
        let mut b = vec![0; 2];
        let mut rand = vec![0; 2];
        let mut out = vec![0; 2];
        if self.is_empty_context() && rhs.is_empty_context() {
            for i in 0..2 {
                self.value[i] += rhs.value[i];
            }
            return Ok(self);
        }

        if self.is_empty_context() {
            return rhs.clone().add_op(&self);
        }
        for (i, random) in rand.iter_mut().enumerate().take(2) {
            *random = get_next_random_from_context(&mut self.ctx.borrow_mut()[i]).unwrap();
        }
        let required = IKosVariable4V::require_reconstruct(&self.ctx.borrow());
        for (i, out_value) in out.iter_mut().enumerate().take(2) {
            if !required || i != 0 {
                *out_value =
                    self.ctx.borrow()[i].ikos_view.out_data[self.ctx.borrow()[i].out_view_ctr];
            }
            self.ctx.borrow_mut()[i].out_view_ctr += 1;
        }
        for i in 0..31 {
            for j in 0..2 {
                a[j] = get_bit!(self.value[j] ^ out[j], i);
                b[j] = get_bit!(rhs.value[j] ^ out[j], i);
            }
            let c = (a[0] & b[1]) ^ (a[1] & b[0]) ^ (get_bit!(rand[1], i));
            if !required {
                if c ^ (a[0] & b[0]) ^ (get_bit!(out[0], i)) ^ (get_bit!(rand[0], i))
                    != get_bit!(out[0], i + 1)
                {
                    return Err(IKosError {
                        error: String::from("IkosVariable4V + operation fail."),
                    });
                }
            } else {
                out[0] = set_bit!(
                    out[0],
                    i + 1,
                    c ^ (a[0] & b[0]) ^ (get_bit!(out[0], i)) ^ (get_bit!(rand[0], i))
                );
            }
        }
        if required {
            self.ctx.borrow_mut()[0].ikos_view.out_data.push(out[0]);
        }
        for (i, &out_value) in out.iter().enumerate().take(2) {
            self.value[i] = self.value[i] ^ rhs.value[i] ^ out_value;
        }
        Ok(self)
    }
}
