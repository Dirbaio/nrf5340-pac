#[doc = "Reader of register DHIT"]
pub type R = crate::R<u32, super::DHIT>;
#[doc = "Reader of field `HITS`"]
pub type HITS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of data cache hits"]
    #[inline(always)]
    pub fn hits(&self) -> HITS_R {
        HITS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
