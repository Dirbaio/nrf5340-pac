#[doc = "Register `ADDRESS[%s]` reader"]
pub struct R(crate::R<ADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ADDRESS_SPEC>> for R {
    fn from(reader: crate::R<ADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDRESS[%s]` writer"]
pub struct W(crate::W<ADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<ADDRESS_SPEC>> for W {
    fn from(writer: crate::W<ADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS` reader - TWI slave address"]
pub struct ADDRESS_R(crate::FieldReader<u8, u8>);
impl ADDRESS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRESS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRESS` writer - TWI slave address"]
pub struct ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - TWI slave address"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - TWI slave address"]
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W {
        ADDRESS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: TWI slave address n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [address](index.html) module"]
pub struct ADDRESS_SPEC;
impl crate::RegisterSpec for ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [address::R](R) reader structure"]
impl crate::Readable for ADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [address::W](W) writer structure"]
impl crate::Writable for ADDRESS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDRESS[%s]
to value 0"]
impl crate::Resettable for ADDRESS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
