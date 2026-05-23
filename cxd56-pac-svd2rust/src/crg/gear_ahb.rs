///Register `GEAR_AHB` reader
pub type R = crate::R<GearAhbSpec>;
///Register `GEAR_AHB` writer
pub type W = crate::W<GearAhbSpec>;
///Field `GEAR_M_AHB` reader -
pub type GearMAhbR = crate::FieldReader;
///Field `GEAR_M_AHB` writer -
pub type GearMAhbW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `GEAR_N_AHB` reader -
pub type GearNAhbR = crate::FieldReader;
///Field `GEAR_N_AHB` writer -
pub type GearNAhbW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6
    #[inline(always)]
    pub fn gear_m_ahb(&self) -> GearMAhbR {
        GearMAhbR::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22
    #[inline(always)]
    pub fn gear_n_ahb(&self) -> GearNAhbR {
        GearNAhbR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:6
    #[inline(always)]
    pub fn gear_m_ahb(&mut self) -> GearMAhbW<'_, GearAhbSpec> {
        GearMAhbW::new(self, 0)
    }
    ///Bits 16:22
    #[inline(always)]
    pub fn gear_n_ahb(&mut self) -> GearNAhbW<'_, GearAhbSpec> {
        GearNAhbW::new(self, 16)
    }
}
/**Gear ratio (n/m) for AHB

You can [`read`](crate::Reg::read) this register and get [`gear_ahb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gear_ahb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GearAhbSpec;
impl crate::RegisterSpec for GearAhbSpec {
    type Ux = u32;
}
///`read()` method returns [`gear_ahb::R`](R) reader structure
impl crate::Readable for GearAhbSpec {}
///`write(|w| ..)` method takes [`gear_ahb::W`](W) writer structure
impl crate::Writable for GearAhbSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GEAR_AHB to value 0x0001_0002
impl crate::Resettable for GearAhbSpec {
    const RESET_VALUE: u32 = 0x0001_0002;
}
