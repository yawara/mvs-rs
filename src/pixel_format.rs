#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum PixelFormat {
    Mono1p,
    Mono2p,
    Mono4p,
    Mono8,
    Mono8_Signed,
    Mono10,
    Mono10_Packed,
    Mono12,
    Mono12_Packed,
    Mono14,
    Mono16,
    BayerGR8,
    BayerRG8,
    BayerGB8,
    BayerBG8,
    BayerGR10,
    BayerRG10,
    BayerGB10,
    BayerBG10,
    BayerGR12,
    BayerRG12,
    BayerGB12,
    BayerBG12,
    BayerGR10_Packed,
    BayerRG10_Packed,
    BayerGB10_Packed,
    BayerBG10_Packed,
    BayerGR12_Packed,
    BayerRG12_Packed,
    BayerGB12_Packed,
    BayerBG12_Packed,
    BayerGR16,
    BayerRG16,
    BayerGB16,
    BayerBG16,
    RGB8_Packed,
    BGR8_Packed,
    RGBA8_Packed,
    BGRA8_Packed,
    RGB10_Packed,
    BGR10_Packed,
    RGB12_Packed,
    BGR12_Packed,
    RGB16_Packed,
    RGB10V1_Packed,
    RGB10V2_Packed,
    RGB12V1_Packed,
    RGB565_Packed,
    BGR565_Packed,
    YUV411_Packed,
    YUV422_Packed,
    YUV422_YUYV_Packed,
    YUV444_Packed,
    YCBCR8_CBYCR,
    YCBCR422_8,
    YCBCR422_8_CBYCRY,
    YCBCR411_8_CBYYCRYY,
    YCBCR601_8_CBYCR,
    YCBCR601_422_8,
    YCBCR601_422_8_CBYCRY,
    YCBCR601_411_8_CBYYCRYY,
    YCBCR709_8_CBYCR,
    YCBCR709_422_8,
    YCBCR709_422_8_CBYCRY,
    YCBCR709_411_8_CBYYCRYY,
    RGB8_Planar,
    RGB10_Planar,
    RGB12_Planar,
    RGB16_Planar,
    Jpeg,
    Coord3D_ABC32f,
    Coord3D_ABC32f_Planar,
    Coord3D_AC32f,
    COORD3D_DEPTH_PLUS_MASK,
    Coord3D_ABC32,
    Coord3D_AB32f,
    Coord3D_AB32,
    Coord3D_AC32f_64,
    Coord3D_AC32f_Planar,
    Coord3D_AC32,
    Coord3D_A32f,
    Coord3D_A32,
    Coord3D_C32f,
    Coord3D_C32,
    Coord3D_ABC16,
}

impl From<i64> for PixelFormat {
    fn from(value: i64) -> Self {
        match value {
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono1p => PixelFormat::Mono1p,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono2p => PixelFormat::Mono2p,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono4p => PixelFormat::Mono4p,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono8 => PixelFormat::Mono8,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono8_Signed => PixelFormat::Mono8_Signed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono10 => PixelFormat::Mono10,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono10_Packed => PixelFormat::Mono10_Packed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono12 => PixelFormat::Mono12,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono12_Packed => PixelFormat::Mono12_Packed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono14 => PixelFormat::Mono14,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono16 => PixelFormat::Mono16,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGR8 => PixelFormat::BayerGR8,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerRG8 => PixelFormat::BayerRG8,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGB8 => PixelFormat::BayerGB8,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerBG8 => PixelFormat::BayerBG8,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGR10 => PixelFormat::BayerGR10,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerRG10 => PixelFormat::BayerRG10,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGB10 => PixelFormat::BayerGB10,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerBG10 => PixelFormat::BayerBG10,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGR12 => PixelFormat::BayerGR12,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerRG12 => PixelFormat::BayerRG12,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGB12 => PixelFormat::BayerGB12,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerBG12 => PixelFormat::BayerBG12,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGR10_Packed => {
                PixelFormat::BayerGR10_Packed
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerRG10_Packed => {
                PixelFormat::BayerRG10_Packed
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGB10_Packed => {
                PixelFormat::BayerGB10_Packed
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerBG10_Packed => {
                PixelFormat::BayerBG10_Packed
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGR12_Packed => {
                PixelFormat::BayerGR12_Packed
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerRG12_Packed => {
                PixelFormat::BayerRG12_Packed
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGB12_Packed => {
                PixelFormat::BayerGB12_Packed
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerBG12_Packed => {
                PixelFormat::BayerBG12_Packed
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGR16 => PixelFormat::BayerGR16,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerRG16 => PixelFormat::BayerRG16,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGB16 => PixelFormat::BayerGB16,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerBG16 => PixelFormat::BayerBG16,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB8_Packed => PixelFormat::RGB8_Packed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BGR8_Packed => PixelFormat::BGR8_Packed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGBA8_Packed => PixelFormat::RGBA8_Packed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BGRA8_Packed => PixelFormat::BGRA8_Packed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB10_Packed => PixelFormat::RGB10_Packed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BGR10_Packed => PixelFormat::BGR10_Packed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB12_Packed => PixelFormat::RGB12_Packed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BGR12_Packed => PixelFormat::BGR12_Packed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB16_Packed => PixelFormat::RGB16_Packed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB10V1_Packed => PixelFormat::RGB10V1_Packed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB10V2_Packed => PixelFormat::RGB10V2_Packed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB12V1_Packed => PixelFormat::RGB12V1_Packed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB565_Packed => PixelFormat::RGB565_Packed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_BGR565_Packed => PixelFormat::BGR565_Packed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_YUV411_Packed => PixelFormat::YUV411_Packed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_YUV422_Packed => PixelFormat::YUV422_Packed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_YUV422_YUYV_Packed => {
                PixelFormat::YUV422_YUYV_Packed
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_YUV444_Packed => PixelFormat::YUV444_Packed,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR8_CBYCR => PixelFormat::YCBCR8_CBYCR,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR422_8 => PixelFormat::YCBCR422_8,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR422_8_CBYCRY => {
                PixelFormat::YCBCR422_8_CBYCRY
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR411_8_CBYYCRYY => {
                PixelFormat::YCBCR411_8_CBYYCRYY
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR601_8_CBYCR => {
                PixelFormat::YCBCR601_8_CBYCR
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR601_422_8 => PixelFormat::YCBCR601_422_8,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR601_422_8_CBYCRY => {
                PixelFormat::YCBCR601_422_8_CBYCRY
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR601_411_8_CBYYCRYY => {
                PixelFormat::YCBCR601_411_8_CBYYCRYY
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR709_8_CBYCR => {
                PixelFormat::YCBCR709_8_CBYCR
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR709_422_8 => PixelFormat::YCBCR709_422_8,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR709_422_8_CBYCRY => {
                PixelFormat::YCBCR709_422_8_CBYCRY
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR709_411_8_CBYYCRYY => {
                PixelFormat::YCBCR709_411_8_CBYYCRYY
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB8_Planar => PixelFormat::RGB8_Planar,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB10_Planar => PixelFormat::RGB10_Planar,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB12_Planar => PixelFormat::RGB12_Planar,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB16_Planar => PixelFormat::RGB16_Planar,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Jpeg => PixelFormat::Jpeg,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_ABC32f => PixelFormat::Coord3D_ABC32f,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_ABC32f_Planar => {
                PixelFormat::Coord3D_ABC32f_Planar
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_AC32f => PixelFormat::Coord3D_AC32f,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_COORD3D_DEPTH_PLUS_MASK => {
                PixelFormat::COORD3D_DEPTH_PLUS_MASK
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_ABC32 => PixelFormat::Coord3D_ABC32,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_AB32f => PixelFormat::Coord3D_AB32f,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_AB32 => PixelFormat::Coord3D_AB32,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_AC32f_64 => {
                PixelFormat::Coord3D_AC32f_64
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_AC32f_Planar => {
                PixelFormat::Coord3D_AC32f_Planar
            }
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_AC32 => PixelFormat::Coord3D_AC32,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_A32f => PixelFormat::Coord3D_A32f,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_A32 => PixelFormat::Coord3D_A32,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_C32f => PixelFormat::Coord3D_C32f,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_C32 => PixelFormat::Coord3D_C32,
            mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_ABC16 => PixelFormat::Coord3D_ABC16,
            code => panic!("Undefined pixel format code: {}", code),
        }
    }
}

impl From<PixelFormat> for i64 {
    fn from(value: PixelFormat) -> Self {
        match value {
            PixelFormat::Mono1p => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono1p,
            PixelFormat::Mono2p => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono2p,
            PixelFormat::Mono4p => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono4p,
            PixelFormat::Mono8 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono8,
            PixelFormat::Mono8_Signed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono8_Signed,
            PixelFormat::Mono10 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono10,
            PixelFormat::Mono10_Packed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono10_Packed,
            PixelFormat::Mono12 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono12,
            PixelFormat::Mono12_Packed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono12_Packed,
            PixelFormat::Mono14 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono14,
            PixelFormat::Mono16 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Mono16,
            PixelFormat::BayerGR8 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGR8,
            PixelFormat::BayerRG8 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerRG8,
            PixelFormat::BayerGB8 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGB8,
            PixelFormat::BayerBG8 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerBG8,
            PixelFormat::BayerGR10 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGR10,
            PixelFormat::BayerRG10 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerRG10,
            PixelFormat::BayerGB10 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGB10,
            PixelFormat::BayerBG10 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerBG10,
            PixelFormat::BayerGR12 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGR12,
            PixelFormat::BayerRG12 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerRG12,
            PixelFormat::BayerGB12 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGB12,
            PixelFormat::BayerBG12 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerBG12,
            PixelFormat::BayerGR10_Packed => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGR10_Packed
            }
            PixelFormat::BayerRG10_Packed => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerRG10_Packed
            }
            PixelFormat::BayerGB10_Packed => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGB10_Packed
            }
            PixelFormat::BayerBG10_Packed => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerBG10_Packed
            }
            PixelFormat::BayerGR12_Packed => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGR12_Packed
            }
            PixelFormat::BayerRG12_Packed => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerRG12_Packed
            }
            PixelFormat::BayerGB12_Packed => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGB12_Packed
            }
            PixelFormat::BayerBG12_Packed => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerBG12_Packed
            }
            PixelFormat::BayerGR16 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGR16,
            PixelFormat::BayerRG16 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerRG16,
            PixelFormat::BayerGB16 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerGB16,
            PixelFormat::BayerBG16 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BayerBG16,
            PixelFormat::RGB8_Packed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB8_Packed,
            PixelFormat::BGR8_Packed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BGR8_Packed,
            PixelFormat::RGBA8_Packed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGBA8_Packed,
            PixelFormat::BGRA8_Packed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BGRA8_Packed,
            PixelFormat::RGB10_Packed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB10_Packed,
            PixelFormat::BGR10_Packed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BGR10_Packed,
            PixelFormat::RGB12_Packed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB12_Packed,
            PixelFormat::BGR12_Packed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BGR12_Packed,
            PixelFormat::RGB16_Packed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB16_Packed,
            PixelFormat::RGB10V1_Packed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB10V1_Packed,
            PixelFormat::RGB10V2_Packed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB10V2_Packed,
            PixelFormat::RGB12V1_Packed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB12V1_Packed,
            PixelFormat::RGB565_Packed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB565_Packed,
            PixelFormat::BGR565_Packed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_BGR565_Packed,
            PixelFormat::YUV411_Packed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_YUV411_Packed,
            PixelFormat::YUV422_Packed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_YUV422_Packed,
            PixelFormat::YUV422_YUYV_Packed => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_YUV422_YUYV_Packed
            }
            PixelFormat::YUV444_Packed => mvs_sys::MvGvspPixelType_PixelType_Gvsp_YUV444_Packed,
            PixelFormat::YCBCR8_CBYCR => mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR8_CBYCR,
            PixelFormat::YCBCR422_8 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR422_8,
            PixelFormat::YCBCR422_8_CBYCRY => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR422_8_CBYCRY
            }
            PixelFormat::YCBCR411_8_CBYYCRYY => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR411_8_CBYYCRYY
            }
            PixelFormat::YCBCR601_8_CBYCR => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR601_8_CBYCR
            }
            PixelFormat::YCBCR601_422_8 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR601_422_8,
            PixelFormat::YCBCR601_422_8_CBYCRY => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR601_422_8_CBYCRY
            }
            PixelFormat::YCBCR601_411_8_CBYYCRYY => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR601_411_8_CBYYCRYY
            }
            PixelFormat::YCBCR709_8_CBYCR => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR709_8_CBYCR
            }
            PixelFormat::YCBCR709_422_8 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR709_422_8,
            PixelFormat::YCBCR709_422_8_CBYCRY => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR709_422_8_CBYCRY
            }
            PixelFormat::YCBCR709_411_8_CBYYCRYY => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_YCBCR709_411_8_CBYYCRYY
            }
            PixelFormat::RGB8_Planar => mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB8_Planar,
            PixelFormat::RGB10_Planar => mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB10_Planar,
            PixelFormat::RGB12_Planar => mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB12_Planar,
            PixelFormat::RGB16_Planar => mvs_sys::MvGvspPixelType_PixelType_Gvsp_RGB16_Planar,
            PixelFormat::Jpeg => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Jpeg,
            PixelFormat::Coord3D_ABC32f => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_ABC32f,
            PixelFormat::Coord3D_ABC32f_Planar => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_ABC32f_Planar
            }
            PixelFormat::Coord3D_AC32f => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_AC32f,
            PixelFormat::COORD3D_DEPTH_PLUS_MASK => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_COORD3D_DEPTH_PLUS_MASK
            }
            PixelFormat::Coord3D_ABC32 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_ABC32,
            PixelFormat::Coord3D_AB32f => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_AB32f,
            PixelFormat::Coord3D_AB32 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_AB32,
            PixelFormat::Coord3D_AC32f_64 => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_AC32f_64
            }
            PixelFormat::Coord3D_AC32f_Planar => {
                mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_AC32f_Planar
            }
            PixelFormat::Coord3D_AC32 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_AC32,
            PixelFormat::Coord3D_A32f => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_A32f,
            PixelFormat::Coord3D_A32 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_A32,
            PixelFormat::Coord3D_C32f => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_C32f,
            PixelFormat::Coord3D_C32 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_C32,
            PixelFormat::Coord3D_ABC16 => mvs_sys::MvGvspPixelType_PixelType_Gvsp_Coord3D_ABC16,
        }
    }
}
