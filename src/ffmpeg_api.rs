use ffmpeg_dev::sys as ffi;
use failure::bail;
use enum_primitive::*;
use std::marker::PhantomData;
use fraction::{Decimal, Fraction};

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(i32)]
    pub enum AVMediaType {
        Unknown = ffi::AVMediaType_AVMEDIA_TYPE_UNKNOWN,
        Video = ffi::AVMediaType_AVMEDIA_TYPE_VIDEO,
        Audio = ffi::AVMediaType_AVMEDIA_TYPE_AUDIO,
        Data = ffi::AVMediaType_AVMEDIA_TYPE_DATA,
        Subtitle = ffi::AVMediaType_AVMEDIA_TYPE_SUBTITLE,
        Attachment = ffi::AVMediaType_AVMEDIA_TYPE_ATTACHMENT,
        Nb = ffi::AVMediaType_AVMEDIA_TYPE_NB,
    }
}

enum_from_primitive! {
    #[allow(non_camel_case_types)]
    #[doc = " Identify the syntax and semantics of the bitstream."]
    #[doc = " The principle is roughly:"]
    #[doc = " Two decoders with the same ID can decode the same streams."]
    #[doc = " Two encoders with the same ID can encode compatible streams."]
    #[doc = " There may be slight deviations from the principle due to implementation"]
    #[doc = " details."]
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum AVCodecID {
        NONE = ffi::AVCodecID_AV_CODEC_ID_NONE,
        MPEG1VIDEO = ffi::AVCodecID_AV_CODEC_ID_MPEG1VIDEO,
        #[doc = "< preferred ID for MPEG-1/2 video decoding"]
        MPEG2VIDEO = ffi::AVCodecID_AV_CODEC_ID_MPEG2VIDEO,
        H261 = ffi::AVCodecID_AV_CODEC_ID_H261,
        H263 = ffi::AVCodecID_AV_CODEC_ID_H263,
        RV10 = ffi::AVCodecID_AV_CODEC_ID_RV10,
        RV20 = ffi::AVCodecID_AV_CODEC_ID_RV20,
        MJPEG = ffi::AVCodecID_AV_CODEC_ID_MJPEG,
        MJPEGB = ffi::AVCodecID_AV_CODEC_ID_MJPEGB,
        LJPEG = ffi::AVCodecID_AV_CODEC_ID_LJPEG,
        SP5X = ffi::AVCodecID_AV_CODEC_ID_SP5X,
        JPEGLS = ffi::AVCodecID_AV_CODEC_ID_JPEGLS,
        MPEG4 = ffi::AVCodecID_AV_CODEC_ID_MPEG4,
        RAWVIDEO = ffi::AVCodecID_AV_CODEC_ID_RAWVIDEO,
        MSMPEG4V1 = ffi::AVCodecID_AV_CODEC_ID_MSMPEG4V1,
        MSMPEG4V2 = ffi::AVCodecID_AV_CODEC_ID_MSMPEG4V2,
        MSMPEG4V3 = ffi::AVCodecID_AV_CODEC_ID_MSMPEG4V3,
        WMV1 = ffi::AVCodecID_AV_CODEC_ID_WMV1,
        WMV2 = ffi::AVCodecID_AV_CODEC_ID_WMV2,
        H263P = ffi::AVCodecID_AV_CODEC_ID_H263P,
        H263I = ffi::AVCodecID_AV_CODEC_ID_H263I,
        FLV1 = ffi::AVCodecID_AV_CODEC_ID_FLV1,
        SVQ1 = ffi::AVCodecID_AV_CODEC_ID_SVQ1,
        SVQ3 = ffi::AVCodecID_AV_CODEC_ID_SVQ3,
        DVVIDEO = ffi::AVCodecID_AV_CODEC_ID_DVVIDEO,
        HUFFYUV = ffi::AVCodecID_AV_CODEC_ID_HUFFYUV,
        CYUV = ffi::AVCodecID_AV_CODEC_ID_CYUV,
        H264 = ffi::AVCodecID_AV_CODEC_ID_H264,
        INDEO3 = ffi::AVCodecID_AV_CODEC_ID_INDEO3,
        VP3 = ffi::AVCodecID_AV_CODEC_ID_VP3,
        THEORA = ffi::AVCodecID_AV_CODEC_ID_THEORA,
        ASV1 = ffi::AVCodecID_AV_CODEC_ID_ASV1,
        ASV2 = ffi::AVCodecID_AV_CODEC_ID_ASV2,
        FFV1 = ffi::AVCodecID_AV_CODEC_ID_FFV1,
        _4XM = ffi::AVCodecID_AV_CODEC_ID_4XM,
        VCR1 = ffi::AVCodecID_AV_CODEC_ID_VCR1,
        CLJR = ffi::AVCodecID_AV_CODEC_ID_CLJR,
        MDEC = ffi::AVCodecID_AV_CODEC_ID_MDEC,
        ROQ = ffi::AVCodecID_AV_CODEC_ID_ROQ,
        INTERPLAY_VIDEO = ffi::AVCodecID_AV_CODEC_ID_INTERPLAY_VIDEO,
        XAN_WC3 = ffi::AVCodecID_AV_CODEC_ID_XAN_WC3,
        XAN_WC4 = ffi::AVCodecID_AV_CODEC_ID_XAN_WC4,
        RPZA = ffi::AVCodecID_AV_CODEC_ID_RPZA,
        CINEPAK = ffi::AVCodecID_AV_CODEC_ID_CINEPAK,
        WS_VQA = ffi::AVCodecID_AV_CODEC_ID_WS_VQA,
        MSRLE = ffi::AVCodecID_AV_CODEC_ID_MSRLE,
        MSVIDEO1 = ffi::AVCodecID_AV_CODEC_ID_MSVIDEO1,
        IDCIN = ffi::AVCodecID_AV_CODEC_ID_IDCIN,
        _8BPS = ffi::AVCodecID_AV_CODEC_ID_8BPS,
        SMC = ffi::AVCodecID_AV_CODEC_ID_SMC,
        FLIC = ffi::AVCodecID_AV_CODEC_ID_FLIC,
        TRUEMOTION1 = ffi::AVCodecID_AV_CODEC_ID_TRUEMOTION1,
        VMDVIDEO = ffi::AVCodecID_AV_CODEC_ID_VMDVIDEO,
        MSZH = ffi::AVCodecID_AV_CODEC_ID_MSZH,
        ZLIB = ffi::AVCodecID_AV_CODEC_ID_ZLIB,
        QTRLE = ffi::AVCodecID_AV_CODEC_ID_QTRLE,
        TSCC = ffi::AVCodecID_AV_CODEC_ID_TSCC,
        ULTI = ffi::AVCodecID_AV_CODEC_ID_ULTI,
        QDRAW = ffi::AVCodecID_AV_CODEC_ID_QDRAW,
        VIXL = ffi::AVCodecID_AV_CODEC_ID_VIXL,
        QPEG = ffi::AVCodecID_AV_CODEC_ID_QPEG,
        PNG = ffi::AVCodecID_AV_CODEC_ID_PNG,
        PPM = ffi::AVCodecID_AV_CODEC_ID_PPM,
        PBM = ffi::AVCodecID_AV_CODEC_ID_PBM,
        PGM = ffi::AVCodecID_AV_CODEC_ID_PGM,
        PGMYUV = ffi::AVCodecID_AV_CODEC_ID_PGMYUV,
        PAM = ffi::AVCodecID_AV_CODEC_ID_PAM,
        FFVHUFF = ffi::AVCodecID_AV_CODEC_ID_FFVHUFF,
        RV30 = ffi::AVCodecID_AV_CODEC_ID_RV30,
        RV40 = ffi::AVCodecID_AV_CODEC_ID_RV40,
        VC1 = ffi::AVCodecID_AV_CODEC_ID_VC1,
        WMV3 = ffi::AVCodecID_AV_CODEC_ID_WMV3,
        LOCO = ffi::AVCodecID_AV_CODEC_ID_LOCO,
        WNV1 = ffi::AVCodecID_AV_CODEC_ID_WNV1,
        AASC = ffi::AVCodecID_AV_CODEC_ID_AASC,
        INDEO2 = ffi::AVCodecID_AV_CODEC_ID_INDEO2,
        FRAPS = ffi::AVCodecID_AV_CODEC_ID_FRAPS,
        TRUEMOTION2 = ffi::AVCodecID_AV_CODEC_ID_TRUEMOTION2,
        BMP = ffi::AVCodecID_AV_CODEC_ID_BMP,
        CSCD = ffi::AVCodecID_AV_CODEC_ID_CSCD,
        MMVIDEO = ffi::AVCodecID_AV_CODEC_ID_MMVIDEO,
        ZMBV = ffi::AVCodecID_AV_CODEC_ID_ZMBV,
        AVS = ffi::AVCodecID_AV_CODEC_ID_AVS,
        SMACKVIDEO = ffi::AVCodecID_AV_CODEC_ID_SMACKVIDEO,
        NUV = ffi::AVCodecID_AV_CODEC_ID_NUV,
        KMVC = ffi::AVCodecID_AV_CODEC_ID_KMVC,
        FLASHSV = ffi::AVCodecID_AV_CODEC_ID_FLASHSV,
        CAVS = ffi::AVCodecID_AV_CODEC_ID_CAVS,
        JPEG2000 = ffi::AVCodecID_AV_CODEC_ID_JPEG2000,
        VMNC = ffi::AVCodecID_AV_CODEC_ID_VMNC,
        VP5 = ffi::AVCodecID_AV_CODEC_ID_VP5,
        VP6 = ffi::AVCodecID_AV_CODEC_ID_VP6,
        VP6F = ffi::AVCodecID_AV_CODEC_ID_VP6F,
        TARGA = ffi::AVCodecID_AV_CODEC_ID_TARGA,
        DSICINVIDEO = ffi::AVCodecID_AV_CODEC_ID_DSICINVIDEO,
        TIERTEXSEQVIDEO = ffi::AVCodecID_AV_CODEC_ID_TIERTEXSEQVIDEO,
        TIFF = ffi::AVCodecID_AV_CODEC_ID_TIFF,
        GIF = ffi::AVCodecID_AV_CODEC_ID_GIF,
        DXA = ffi::AVCodecID_AV_CODEC_ID_DXA,
        DNXHD = ffi::AVCodecID_AV_CODEC_ID_DNXHD,
        THP = ffi::AVCodecID_AV_CODEC_ID_THP,
        SGI = ffi::AVCodecID_AV_CODEC_ID_SGI,
        C93 = ffi::AVCodecID_AV_CODEC_ID_C93,
        BETHSOFTVID = ffi::AVCodecID_AV_CODEC_ID_BETHSOFTVID,
        PTX = ffi::AVCodecID_AV_CODEC_ID_PTX,
        TXD = ffi::AVCodecID_AV_CODEC_ID_TXD,
        VP6A = ffi::AVCodecID_AV_CODEC_ID_VP6A,
        AMV = ffi::AVCodecID_AV_CODEC_ID_AMV,
        VB = ffi::AVCodecID_AV_CODEC_ID_VB,
        PCX = ffi::AVCodecID_AV_CODEC_ID_PCX,
        SUNRAST = ffi::AVCodecID_AV_CODEC_ID_SUNRAST,
        INDEO4 = ffi::AVCodecID_AV_CODEC_ID_INDEO4,
        INDEO5 = ffi::AVCodecID_AV_CODEC_ID_INDEO5,
        MIMIC = ffi::AVCodecID_AV_CODEC_ID_MIMIC,
        RL2 = ffi::AVCodecID_AV_CODEC_ID_RL2,
        ESCAPE124 = ffi::AVCodecID_AV_CODEC_ID_ESCAPE124,
        DIRAC = ffi::AVCodecID_AV_CODEC_ID_DIRAC,
        BFI = ffi::AVCodecID_AV_CODEC_ID_BFI,
        CMV = ffi::AVCodecID_AV_CODEC_ID_CMV,
        MOTIONPIXELS = ffi::AVCodecID_AV_CODEC_ID_MOTIONPIXELS,
        TGV = ffi::AVCodecID_AV_CODEC_ID_TGV,
        TGQ = ffi::AVCodecID_AV_CODEC_ID_TGQ,
        TQI = ffi::AVCodecID_AV_CODEC_ID_TQI,
        AURA = ffi::AVCodecID_AV_CODEC_ID_AURA,
        AURA2 = ffi::AVCodecID_AV_CODEC_ID_AURA2,
        V210X = ffi::AVCodecID_AV_CODEC_ID_V210X,
        TMV = ffi::AVCodecID_AV_CODEC_ID_TMV,
        V210 = ffi::AVCodecID_AV_CODEC_ID_V210,
        DPX = ffi::AVCodecID_AV_CODEC_ID_DPX,
        MAD = ffi::AVCodecID_AV_CODEC_ID_MAD,
        FRWU = ffi::AVCodecID_AV_CODEC_ID_FRWU,
        FLASHSV2 = ffi::AVCodecID_AV_CODEC_ID_FLASHSV2,
        CDGRAPHICS = ffi::AVCodecID_AV_CODEC_ID_CDGRAPHICS,
        R210 = ffi::AVCodecID_AV_CODEC_ID_R210,
        ANM = ffi::AVCodecID_AV_CODEC_ID_ANM,
        BINKVIDEO = ffi::AVCodecID_AV_CODEC_ID_BINKVIDEO,
        IFF_ILBM = ffi::AVCodecID_AV_CODEC_ID_IFF_ILBM,
        KGV1 = ffi::AVCodecID_AV_CODEC_ID_KGV1,
        YOP = ffi::AVCodecID_AV_CODEC_ID_YOP,
        VP8 = ffi::AVCodecID_AV_CODEC_ID_VP8,
        PICTOR = ffi::AVCodecID_AV_CODEC_ID_PICTOR,
        ANSI = ffi::AVCodecID_AV_CODEC_ID_ANSI,
        A64_MULTI = ffi::AVCodecID_AV_CODEC_ID_A64_MULTI,
        A64_MULTI5 = ffi::AVCodecID_AV_CODEC_ID_A64_MULTI5,
        R10K = ffi::AVCodecID_AV_CODEC_ID_R10K,
        MXPEG = ffi::AVCodecID_AV_CODEC_ID_MXPEG,
        LAGARITH = ffi::AVCodecID_AV_CODEC_ID_LAGARITH,
        PRORES = ffi::AVCodecID_AV_CODEC_ID_PRORES,
        JV = ffi::AVCodecID_AV_CODEC_ID_JV,
        DFA = ffi::AVCodecID_AV_CODEC_ID_DFA,
        WMV3IMAGE = ffi::AVCodecID_AV_CODEC_ID_WMV3IMAGE,
        VC1IMAGE = ffi::AVCodecID_AV_CODEC_ID_VC1IMAGE,
        UTVIDEO = ffi::AVCodecID_AV_CODEC_ID_UTVIDEO,
        BMV_VIDEO = ffi::AVCodecID_AV_CODEC_ID_BMV_VIDEO,
        VBLE = ffi::AVCodecID_AV_CODEC_ID_VBLE,
        DXTORY = ffi::AVCodecID_AV_CODEC_ID_DXTORY,
        V410 = ffi::AVCodecID_AV_CODEC_ID_V410,
        XWD = ffi::AVCodecID_AV_CODEC_ID_XWD,
        CDXL = ffi::AVCodecID_AV_CODEC_ID_CDXL,
        XBM = ffi::AVCodecID_AV_CODEC_ID_XBM,
        ZEROCODEC = ffi::AVCodecID_AV_CODEC_ID_ZEROCODEC,
        MSS1 = ffi::AVCodecID_AV_CODEC_ID_MSS1,
        MSA1 = ffi::AVCodecID_AV_CODEC_ID_MSA1,
        TSCC2 = ffi::AVCodecID_AV_CODEC_ID_TSCC2,
        MTS2 = ffi::AVCodecID_AV_CODEC_ID_MTS2,
        CLLC = ffi::AVCodecID_AV_CODEC_ID_CLLC,
        MSS2 = ffi::AVCodecID_AV_CODEC_ID_MSS2,
        VP9 = ffi::AVCodecID_AV_CODEC_ID_VP9,
        AIC = ffi::AVCodecID_AV_CODEC_ID_AIC,
        ESCAPE130 = ffi::AVCodecID_AV_CODEC_ID_ESCAPE130,
        G2M = ffi::AVCodecID_AV_CODEC_ID_G2M,
        WEBP = ffi::AVCodecID_AV_CODEC_ID_WEBP,
        HNM4_VIDEO = ffi::AVCodecID_AV_CODEC_ID_HNM4_VIDEO,
        HEVC = ffi::AVCodecID_AV_CODEC_ID_HEVC,
        FIC = ffi::AVCodecID_AV_CODEC_ID_FIC,
        ALIAS_PIX = ffi::AVCodecID_AV_CODEC_ID_ALIAS_PIX,
        BRENDER_PIX = ffi::AVCodecID_AV_CODEC_ID_BRENDER_PIX,
        PAF_VIDEO = ffi::AVCodecID_AV_CODEC_ID_PAF_VIDEO,
        EXR = ffi::AVCodecID_AV_CODEC_ID_EXR,
        VP7 = ffi::AVCodecID_AV_CODEC_ID_VP7,
        SANM = ffi::AVCodecID_AV_CODEC_ID_SANM,
        SGIRLE = ffi::AVCodecID_AV_CODEC_ID_SGIRLE,
        MVC1 = ffi::AVCodecID_AV_CODEC_ID_MVC1,
        MVC2 = ffi::AVCodecID_AV_CODEC_ID_MVC2,
        HQX = ffi::AVCodecID_AV_CODEC_ID_HQX,
        TDSC = ffi::AVCodecID_AV_CODEC_ID_TDSC,
        HQ_HQA = ffi::AVCodecID_AV_CODEC_ID_HQ_HQA,
        HAP = ffi::AVCodecID_AV_CODEC_ID_HAP,
        DDS = ffi::AVCodecID_AV_CODEC_ID_DDS,
        DXV = ffi::AVCodecID_AV_CODEC_ID_DXV,
        SCREENPRESSO = ffi::AVCodecID_AV_CODEC_ID_SCREENPRESSO,
        RSCC = ffi::AVCodecID_AV_CODEC_ID_RSCC,
        AVS2 = ffi::AVCodecID_AV_CODEC_ID_AVS2,
        Y41P = ffi::AVCodecID_AV_CODEC_ID_Y41P,
        AVRP = ffi::AVCodecID_AV_CODEC_ID_AVRP,
        _012V = ffi::AVCodecID_AV_CODEC_ID_012V,
        AVUI = ffi::AVCodecID_AV_CODEC_ID_AVUI,
        AYUV = ffi::AVCodecID_AV_CODEC_ID_AYUV,
        TARGA_Y216 = ffi::AVCodecID_AV_CODEC_ID_TARGA_Y216,
        V308 = ffi::AVCodecID_AV_CODEC_ID_V308,
        V408 = ffi::AVCodecID_AV_CODEC_ID_V408,
        YUV4 = ffi::AVCodecID_AV_CODEC_ID_YUV4,
        AVRN = ffi::AVCodecID_AV_CODEC_ID_AVRN,
        CPIA = ffi::AVCodecID_AV_CODEC_ID_CPIA,
        XFACE = ffi::AVCodecID_AV_CODEC_ID_XFACE,
        SNOW = ffi::AVCodecID_AV_CODEC_ID_SNOW,
        SMVJPEG = ffi::AVCodecID_AV_CODEC_ID_SMVJPEG,
        APNG = ffi::AVCodecID_AV_CODEC_ID_APNG,
        DAALA = ffi::AVCodecID_AV_CODEC_ID_DAALA,
        CFHD = ffi::AVCodecID_AV_CODEC_ID_CFHD,
        TRUEMOTION2RT = ffi::AVCodecID_AV_CODEC_ID_TRUEMOTION2RT,
        M101 = ffi::AVCodecID_AV_CODEC_ID_M101,
        MAGICYUV = ffi::AVCodecID_AV_CODEC_ID_MAGICYUV,
        SHEERVIDEO = ffi::AVCodecID_AV_CODEC_ID_SHEERVIDEO,
        YLC = ffi::AVCodecID_AV_CODEC_ID_YLC,
        PSD = ffi::AVCodecID_AV_CODEC_ID_PSD,
        PIXLET = ffi::AVCodecID_AV_CODEC_ID_PIXLET,
        SPEEDHQ = ffi::AVCodecID_AV_CODEC_ID_SPEEDHQ,
        FMVC = ffi::AVCodecID_AV_CODEC_ID_FMVC,
        SCPR = ffi::AVCodecID_AV_CODEC_ID_SCPR,
        CLEARVIDEO = ffi::AVCodecID_AV_CODEC_ID_CLEARVIDEO,
        XPM = ffi::AVCodecID_AV_CODEC_ID_XPM,
        AV1 = ffi::AVCodecID_AV_CODEC_ID_AV1,
        BITPACKED = ffi::AVCodecID_AV_CODEC_ID_BITPACKED,
        MSCC = ffi::AVCodecID_AV_CODEC_ID_MSCC,
        SRGC = ffi::AVCodecID_AV_CODEC_ID_SRGC,
        SVG = ffi::AVCodecID_AV_CODEC_ID_SVG,
        GDV = ffi::AVCodecID_AV_CODEC_ID_GDV,
        FITS = ffi::AVCodecID_AV_CODEC_ID_FITS,
        IMM4 = ffi::AVCodecID_AV_CODEC_ID_IMM4,
        PROSUMER = ffi::AVCodecID_AV_CODEC_ID_PROSUMER,
        MWSC = ffi::AVCodecID_AV_CODEC_ID_MWSC,
        WCMV = ffi::AVCodecID_AV_CODEC_ID_WCMV,
        RASC = ffi::AVCodecID_AV_CODEC_ID_RASC,
        HYMT = ffi::AVCodecID_AV_CODEC_ID_HYMT,
        ARBC = ffi::AVCodecID_AV_CODEC_ID_ARBC,
        AGM = ffi::AVCodecID_AV_CODEC_ID_AGM,
        LSCR = ffi::AVCodecID_AV_CODEC_ID_LSCR,
        VP4 = ffi::AVCodecID_AV_CODEC_ID_VP4,
        PCM_S16LE = ffi::AVCodecID_AV_CODEC_ID_PCM_S16LE,
        PCM_S16BE = ffi::AVCodecID_AV_CODEC_ID_PCM_S16BE,
        PCM_U16LE = ffi::AVCodecID_AV_CODEC_ID_PCM_U16LE,
        PCM_U16BE = ffi::AVCodecID_AV_CODEC_ID_PCM_U16BE,
        PCM_S8 = ffi::AVCodecID_AV_CODEC_ID_PCM_S8,
        PCM_U8 = ffi::AVCodecID_AV_CODEC_ID_PCM_U8,
        PCM_MULAW = ffi::AVCodecID_AV_CODEC_ID_PCM_MULAW,
        PCM_ALAW = ffi::AVCodecID_AV_CODEC_ID_PCM_ALAW,
        PCM_S32LE = ffi::AVCodecID_AV_CODEC_ID_PCM_S32LE,
        PCM_S32BE = ffi::AVCodecID_AV_CODEC_ID_PCM_S32BE,
        PCM_U32LE = ffi::AVCodecID_AV_CODEC_ID_PCM_U32LE,
        PCM_U32BE = ffi::AVCodecID_AV_CODEC_ID_PCM_U32BE,
        PCM_S24LE = ffi::AVCodecID_AV_CODEC_ID_PCM_S24LE,
        PCM_S24BE = ffi::AVCodecID_AV_CODEC_ID_PCM_S24BE,
        PCM_U24LE = ffi::AVCodecID_AV_CODEC_ID_PCM_U24LE,
        PCM_U24BE = ffi::AVCodecID_AV_CODEC_ID_PCM_U24BE,
        PCM_S24DAUD = ffi::AVCodecID_AV_CODEC_ID_PCM_S24DAUD,
        PCM_ZORK = ffi::AVCodecID_AV_CODEC_ID_PCM_ZORK,
        PCM_S16LE_PLANAR = ffi::AVCodecID_AV_CODEC_ID_PCM_S16LE_PLANAR,
        PCM_DVD = ffi::AVCodecID_AV_CODEC_ID_PCM_DVD,
        PCM_F32BE = ffi::AVCodecID_AV_CODEC_ID_PCM_F32BE,
        PCM_F32LE = ffi::AVCodecID_AV_CODEC_ID_PCM_F32LE,
        PCM_F64BE = ffi::AVCodecID_AV_CODEC_ID_PCM_F64BE,
        PCM_F64LE = ffi::AVCodecID_AV_CODEC_ID_PCM_F64LE,
        PCM_BLURAY = ffi::AVCodecID_AV_CODEC_ID_PCM_BLURAY,
        PCM_LXF = ffi::AVCodecID_AV_CODEC_ID_PCM_LXF,
        S302M = ffi::AVCodecID_AV_CODEC_ID_S302M,
        PCM_S8_PLANAR = ffi::AVCodecID_AV_CODEC_ID_PCM_S8_PLANAR,
        PCM_S24LE_PLANAR = ffi::AVCodecID_AV_CODEC_ID_PCM_S24LE_PLANAR,
        PCM_S32LE_PLANAR = ffi::AVCodecID_AV_CODEC_ID_PCM_S32LE_PLANAR,
        PCM_S16BE_PLANAR = ffi::AVCodecID_AV_CODEC_ID_PCM_S16BE_PLANAR,
        PCM_S64LE = ffi::AVCodecID_AV_CODEC_ID_PCM_S64LE,
        PCM_S64BE = ffi::AVCodecID_AV_CODEC_ID_PCM_S64BE,
        PCM_F16LE = ffi::AVCodecID_AV_CODEC_ID_PCM_F16LE,
        PCM_F24LE = ffi::AVCodecID_AV_CODEC_ID_PCM_F24LE,
        PCM_VIDC = ffi::AVCodecID_AV_CODEC_ID_PCM_VIDC,
        ADPCM_IMA_QT = ffi::AVCodecID_AV_CODEC_ID_ADPCM_IMA_QT,
        ADPCM_IMA_WAV = ffi::AVCodecID_AV_CODEC_ID_ADPCM_IMA_WAV,
        ADPCM_IMA_DK3 = ffi::AVCodecID_AV_CODEC_ID_ADPCM_IMA_DK3,
        ADPCM_IMA_DK4 = ffi::AVCodecID_AV_CODEC_ID_ADPCM_IMA_DK4,
        ADPCM_IMA_WS = ffi::AVCodecID_AV_CODEC_ID_ADPCM_IMA_WS,
        ADPCM_IMA_SMJPEG = ffi::AVCodecID_AV_CODEC_ID_ADPCM_IMA_SMJPEG,
        ADPCM_MS = ffi::AVCodecID_AV_CODEC_ID_ADPCM_MS,
        ADPCM_4XM = ffi::AVCodecID_AV_CODEC_ID_ADPCM_4XM,
        ADPCM_XA = ffi::AVCodecID_AV_CODEC_ID_ADPCM_XA,
        ADPCM_ADX = ffi::AVCodecID_AV_CODEC_ID_ADPCM_ADX,
        ADPCM_EA = ffi::AVCodecID_AV_CODEC_ID_ADPCM_EA,
        ADPCM_G726 = ffi::AVCodecID_AV_CODEC_ID_ADPCM_G726,
        ADPCM_CT = ffi::AVCodecID_AV_CODEC_ID_ADPCM_CT,
        ADPCM_SWF = ffi::AVCodecID_AV_CODEC_ID_ADPCM_SWF,
        ADPCM_YAMAHA = ffi::AVCodecID_AV_CODEC_ID_ADPCM_YAMAHA,
        ADPCM_SBPRO_4 = ffi::AVCodecID_AV_CODEC_ID_ADPCM_SBPRO_4,
        ADPCM_SBPRO_3 = ffi::AVCodecID_AV_CODEC_ID_ADPCM_SBPRO_3,
        ADPCM_SBPRO_2 = ffi::AVCodecID_AV_CODEC_ID_ADPCM_SBPRO_2,
        ADPCM_THP = ffi::AVCodecID_AV_CODEC_ID_ADPCM_THP,
        ADPCM_IMA_AMV = ffi::AVCodecID_AV_CODEC_ID_ADPCM_IMA_AMV,
        ADPCM_EA_R1 = ffi::AVCodecID_AV_CODEC_ID_ADPCM_EA_R1,
        ADPCM_EA_R3 = ffi::AVCodecID_AV_CODEC_ID_ADPCM_EA_R3,
        ADPCM_EA_R2 = ffi::AVCodecID_AV_CODEC_ID_ADPCM_EA_R2,
        ADPCM_IMA_EA_SEAD = ffi::AVCodecID_AV_CODEC_ID_ADPCM_IMA_EA_SEAD,
        ADPCM_IMA_EA_EACS = ffi::AVCodecID_AV_CODEC_ID_ADPCM_IMA_EA_EACS,
        ADPCM_EA_XAS = ffi::AVCodecID_AV_CODEC_ID_ADPCM_EA_XAS,
        ADPCM_EA_MAXIS_XA = ffi::AVCodecID_AV_CODEC_ID_ADPCM_EA_MAXIS_XA,
        ADPCM_IMA_ISS = ffi::AVCodecID_AV_CODEC_ID_ADPCM_IMA_ISS,
        ADPCM_G722 = ffi::AVCodecID_AV_CODEC_ID_ADPCM_G722,
        ADPCM_IMA_APC = ffi::AVCodecID_AV_CODEC_ID_ADPCM_IMA_APC,
        ADPCM_VIMA = ffi::AVCodecID_AV_CODEC_ID_ADPCM_VIMA,
        ADPCM_AFC = ffi::AVCodecID_AV_CODEC_ID_ADPCM_AFC,
        ADPCM_IMA_OKI = ffi::AVCodecID_AV_CODEC_ID_ADPCM_IMA_OKI,
        ADPCM_DTK = ffi::AVCodecID_AV_CODEC_ID_ADPCM_DTK,
        ADPCM_IMA_RAD = ffi::AVCodecID_AV_CODEC_ID_ADPCM_IMA_RAD,
        ADPCM_G726LE = ffi::AVCodecID_AV_CODEC_ID_ADPCM_G726LE,
        ADPCM_THP_LE = ffi::AVCodecID_AV_CODEC_ID_ADPCM_THP_LE,
        ADPCM_PSX = ffi::AVCodecID_AV_CODEC_ID_ADPCM_PSX,
        ADPCM_AICA = ffi::AVCodecID_AV_CODEC_ID_ADPCM_AICA,
        ADPCM_IMA_DAT4 = ffi::AVCodecID_AV_CODEC_ID_ADPCM_IMA_DAT4,
        ADPCM_MTAF = ffi::AVCodecID_AV_CODEC_ID_ADPCM_MTAF,
        ADPCM_AGM = ffi::AVCodecID_AV_CODEC_ID_ADPCM_AGM,
        AMR_NB = ffi::AVCodecID_AV_CODEC_ID_AMR_NB,
        AMR_WB = ffi::AVCodecID_AV_CODEC_ID_AMR_WB,
        RA_144 = ffi::AVCodecID_AV_CODEC_ID_RA_144,
        RA_288 = ffi::AVCodecID_AV_CODEC_ID_RA_288,
        ROQ_DPCM = ffi::AVCodecID_AV_CODEC_ID_ROQ_DPCM,
        INTERPLAY_DPCM = ffi::AVCodecID_AV_CODEC_ID_INTERPLAY_DPCM,
        XAN_DPCM = ffi::AVCodecID_AV_CODEC_ID_XAN_DPCM,
        SOL_DPCM = ffi::AVCodecID_AV_CODEC_ID_SOL_DPCM,
        SDX2_DPCM = ffi::AVCodecID_AV_CODEC_ID_SDX2_DPCM,
        GREMLIN_DPCM = ffi::AVCodecID_AV_CODEC_ID_GREMLIN_DPCM,
        MP2 = ffi::AVCodecID_AV_CODEC_ID_MP2,
        #[doc = "< preferred ID for decoding MPEG audio layer 1, 2 or 3"]
        MP3 = ffi::AVCodecID_AV_CODEC_ID_MP3,
        AAC = ffi::AVCodecID_AV_CODEC_ID_AAC,
        AC3 = ffi::AVCodecID_AV_CODEC_ID_AC3,
        DTS = ffi::AVCodecID_AV_CODEC_ID_DTS,
        VORBIS = ffi::AVCodecID_AV_CODEC_ID_VORBIS,
        DVAUDIO = ffi::AVCodecID_AV_CODEC_ID_DVAUDIO,
        WMAV1 = ffi::AVCodecID_AV_CODEC_ID_WMAV1,
        WMAV2 = ffi::AVCodecID_AV_CODEC_ID_WMAV2,
        MACE3 = ffi::AVCodecID_AV_CODEC_ID_MACE3,
        MACE6 = ffi::AVCodecID_AV_CODEC_ID_MACE6,
        VMDAUDIO = ffi::AVCodecID_AV_CODEC_ID_VMDAUDIO,
        FLAC = ffi::AVCodecID_AV_CODEC_ID_FLAC,
        MP3ADU = ffi::AVCodecID_AV_CODEC_ID_MP3ADU,
        MP3ON4 = ffi::AVCodecID_AV_CODEC_ID_MP3ON4,
        SHORTEN = ffi::AVCodecID_AV_CODEC_ID_SHORTEN,
        ALAC = ffi::AVCodecID_AV_CODEC_ID_ALAC,
        WESTWOOD_SND1 = ffi::AVCodecID_AV_CODEC_ID_WESTWOOD_SND1,
        #[doc = "< as in Berlin toast format"]
        GSM = ffi::AVCodecID_AV_CODEC_ID_GSM,
        QDM2 = ffi::AVCodecID_AV_CODEC_ID_QDM2,
        COOK = ffi::AVCodecID_AV_CODEC_ID_COOK,
        TRUESPEECH = ffi::AVCodecID_AV_CODEC_ID_TRUESPEECH,
        TTA = ffi::AVCodecID_AV_CODEC_ID_TTA,
        SMACKAUDIO = ffi::AVCodecID_AV_CODEC_ID_SMACKAUDIO,
        QCELP = ffi::AVCodecID_AV_CODEC_ID_QCELP,
        WAVPACK = ffi::AVCodecID_AV_CODEC_ID_WAVPACK,
        DSICINAUDIO = ffi::AVCodecID_AV_CODEC_ID_DSICINAUDIO,
        IMC = ffi::AVCodecID_AV_CODEC_ID_IMC,
        MUSEPACK7 = ffi::AVCodecID_AV_CODEC_ID_MUSEPACK7,
        MLP = ffi::AVCodecID_AV_CODEC_ID_MLP,
        GSM_MS = ffi::AVCodecID_AV_CODEC_ID_GSM_MS,
        ATRAC3 = ffi::AVCodecID_AV_CODEC_ID_ATRAC3,
        APE = ffi::AVCodecID_AV_CODEC_ID_APE,
        NELLYMOSER = ffi::AVCodecID_AV_CODEC_ID_NELLYMOSER,
        MUSEPACK8 = ffi::AVCodecID_AV_CODEC_ID_MUSEPACK8,
        SPEEX = ffi::AVCodecID_AV_CODEC_ID_SPEEX,
        WMAVOICE = ffi::AVCodecID_AV_CODEC_ID_WMAVOICE,
        WMAPRO = ffi::AVCodecID_AV_CODEC_ID_WMAPRO,
        WMALOSSLESS = ffi::AVCodecID_AV_CODEC_ID_WMALOSSLESS,
        ATRAC3P = ffi::AVCodecID_AV_CODEC_ID_ATRAC3P,
        EAC3 = ffi::AVCodecID_AV_CODEC_ID_EAC3,
        SIPR = ffi::AVCodecID_AV_CODEC_ID_SIPR,
        MP1 = ffi::AVCodecID_AV_CODEC_ID_MP1,
        TWINVQ = ffi::AVCodecID_AV_CODEC_ID_TWINVQ,
        TRUEHD = ffi::AVCodecID_AV_CODEC_ID_TRUEHD,
        MP4ALS = ffi::AVCodecID_AV_CODEC_ID_MP4ALS,
        ATRAC1 = ffi::AVCodecID_AV_CODEC_ID_ATRAC1,
        BINKAUDIO_RDFT = ffi::AVCodecID_AV_CODEC_ID_BINKAUDIO_RDFT,
        BINKAUDIO_DCT = ffi::AVCodecID_AV_CODEC_ID_BINKAUDIO_DCT,
        AAC_LATM = ffi::AVCodecID_AV_CODEC_ID_AAC_LATM,
        QDMC = ffi::AVCodecID_AV_CODEC_ID_QDMC,
        CELT = ffi::AVCodecID_AV_CODEC_ID_CELT,
        G723_1 = ffi::AVCodecID_AV_CODEC_ID_G723_1,
        G729 = ffi::AVCodecID_AV_CODEC_ID_G729,
        _8SVX_EXP = ffi::AVCodecID_AV_CODEC_ID_8SVX_EXP,
        _8SVX_FIB = ffi::AVCodecID_AV_CODEC_ID_8SVX_FIB,
        BMV_AUDIO = ffi::AVCodecID_AV_CODEC_ID_BMV_AUDIO,
        RALF = ffi::AVCodecID_AV_CODEC_ID_RALF,
        IAC = ffi::AVCodecID_AV_CODEC_ID_IAC,
        ILBC = ffi::AVCodecID_AV_CODEC_ID_ILBC,
        OPUS = ffi::AVCodecID_AV_CODEC_ID_OPUS,
        COMFORT_NOISE = ffi::AVCodecID_AV_CODEC_ID_COMFORT_NOISE,
        TAK = ffi::AVCodecID_AV_CODEC_ID_TAK,
        METASOUND = ffi::AVCodecID_AV_CODEC_ID_METASOUND,
        PAF_AUDIO = ffi::AVCodecID_AV_CODEC_ID_PAF_AUDIO,
        ON2AVC = ffi::AVCodecID_AV_CODEC_ID_ON2AVC,
        DSS_SP = ffi::AVCodecID_AV_CODEC_ID_DSS_SP,
        CODEC2 = ffi::AVCodecID_AV_CODEC_ID_CODEC2,
        FFWAVESYNTH = ffi::AVCodecID_AV_CODEC_ID_FFWAVESYNTH,
        SONIC = ffi::AVCodecID_AV_CODEC_ID_SONIC,
        SONIC_LS = ffi::AVCodecID_AV_CODEC_ID_SONIC_LS,
        EVRC = ffi::AVCodecID_AV_CODEC_ID_EVRC,
        SMV = ffi::AVCodecID_AV_CODEC_ID_SMV,
        DSD_LSBF = ffi::AVCodecID_AV_CODEC_ID_DSD_LSBF,
        DSD_MSBF = ffi::AVCodecID_AV_CODEC_ID_DSD_MSBF,
        DSD_LSBF_PLANAR = ffi::AVCodecID_AV_CODEC_ID_DSD_LSBF_PLANAR,
        DSD_MSBF_PLANAR = ffi::AVCodecID_AV_CODEC_ID_DSD_MSBF_PLANAR,
        _4GV = ffi::AVCodecID_AV_CODEC_ID_4GV,
        INTERPLAY_ACM = ffi::AVCodecID_AV_CODEC_ID_INTERPLAY_ACM,
        XMA1 = ffi::AVCodecID_AV_CODEC_ID_XMA1,
        XMA2 = ffi::AVCodecID_AV_CODEC_ID_XMA2,
        DST = ffi::AVCodecID_AV_CODEC_ID_DST,
        ATRAC3AL = ffi::AVCodecID_AV_CODEC_ID_ATRAC3AL,
        ATRAC3PAL = ffi::AVCodecID_AV_CODEC_ID_ATRAC3PAL,
        DOLBY_E = ffi::AVCodecID_AV_CODEC_ID_DOLBY_E,
        APTX = ffi::AVCodecID_AV_CODEC_ID_APTX,
        APTX_HD = ffi::AVCodecID_AV_CODEC_ID_APTX_HD,
        SBC = ffi::AVCodecID_AV_CODEC_ID_SBC,
        ATRAC9 = ffi::AVCodecID_AV_CODEC_ID_ATRAC9,
        HCOM = ffi::AVCodecID_AV_CODEC_ID_HCOM,
        DVD_SUBTITLE = ffi::AVCodecID_AV_CODEC_ID_DVD_SUBTITLE,
        DVB_SUBTITLE = ffi::AVCodecID_AV_CODEC_ID_DVB_SUBTITLE,
        #[doc = "< raw UTF-8 text"]
        TEXT = ffi::AVCodecID_AV_CODEC_ID_TEXT,
        XSUB = ffi::AVCodecID_AV_CODEC_ID_XSUB,
        SSA = ffi::AVCodecID_AV_CODEC_ID_SSA,
        MOV_TEXT = ffi::AVCodecID_AV_CODEC_ID_MOV_TEXT,
        HDMV_PGS_SUBTITLE = ffi::AVCodecID_AV_CODEC_ID_HDMV_PGS_SUBTITLE,
        DVB_TELETEXT = ffi::AVCodecID_AV_CODEC_ID_DVB_TELETEXT,
        SRT = ffi::AVCodecID_AV_CODEC_ID_SRT,
        MICRODVD = ffi::AVCodecID_AV_CODEC_ID_MICRODVD,
        EIA_608 = ffi::AVCodecID_AV_CODEC_ID_EIA_608,
        JACOSUB = ffi::AVCodecID_AV_CODEC_ID_JACOSUB,
        SAMI = ffi::AVCodecID_AV_CODEC_ID_SAMI,
        REALTEXT = ffi::AVCodecID_AV_CODEC_ID_REALTEXT,
        STL = ffi::AVCodecID_AV_CODEC_ID_STL,
        SUBVIEWER1 = ffi::AVCodecID_AV_CODEC_ID_SUBVIEWER1,
        SUBVIEWER = ffi::AVCodecID_AV_CODEC_ID_SUBVIEWER,
        SUBRIP = ffi::AVCodecID_AV_CODEC_ID_SUBRIP,
        WEBVTT = ffi::AVCodecID_AV_CODEC_ID_WEBVTT,
        MPL2 = ffi::AVCodecID_AV_CODEC_ID_MPL2,
        VPLAYER = ffi::AVCodecID_AV_CODEC_ID_VPLAYER,
        PJS = ffi::AVCodecID_AV_CODEC_ID_PJS,
        ASS = ffi::AVCodecID_AV_CODEC_ID_ASS,
        HDMV_TEXT_SUBTITLE = ffi::AVCodecID_AV_CODEC_ID_HDMV_TEXT_SUBTITLE,
        TTML = ffi::AVCodecID_AV_CODEC_ID_TTML,
        ARIB_CAPTION = ffi::AVCodecID_AV_CODEC_ID_ARIB_CAPTION
    }
}

// TODO: Use proper errors (with struct etc) for this
enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(i32)]
    pub enum AVErrorKind {
        Unknown = ffi::AVERROR_EXPERIMENTAL,
        InputChanged = ffi::AVERROR_INPUT_CHANGED,
        OutputChanged = ffi::AVERROR_OUTPUT_CHANGED
    }
}

pub struct AVFormatContext {
    base: *mut ffi::AVFormatContext,
}

impl<'a> AVFormatContext {
    pub fn new() -> Result<Self, failure::Error> {
        let base = unsafe { ffi::avformat_alloc_context() };
        if base.is_null() {
            bail!("avformat_alloc_context() failed");
        }
        Ok(AVFormatContext { base })
    }

    // TODO: Just for testing
    pub unsafe fn raw(&mut self) -> *mut ffi::AVFormatContext {
        self.base
    }

    pub fn open_input(&mut self, path: &str) -> Result<(), failure::Error> {
        match unsafe {
            ffi::avformat_open_input(
                &mut self.base,
                std::ffi::CString::new(path)
                    .map_err(|_| failure::format_err!("Could not convert path to c string"))?
                    .as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        } {
            0 => Ok(()),
            _ => bail!("Could not open input")
        }
    }

    pub fn streams(&self) -> Vec<AVStream> {
        return unsafe {
            std::slice::from_raw_parts(
                (*self.base).streams,
                (*self.base).nb_streams as usize,
            )
        }
            .iter()
            .map(|stream| {
                AVStream::new(unsafe { (*stream).as_mut() }.expect("not null"), self)
            })
            .collect();
    }
}

impl Drop for AVFormatContext {
    fn drop(&mut self) {
        unsafe { ffi::avformat_free_context(self.base) }
    }
}

enum_from_primitive! {
    # [derive(Debug, Copy, Clone, PartialEq)]
    # [repr(i32)]
    pub enum AVDiscard {
        # [doc = "< discard nothing"]
        None = ffi::AVDiscard_AVDISCARD_NONE,
        # [doc = "< discard useless packets like 0 size packets in avi"]
        Default =ffi::AVDiscard_AVDISCARD_DEFAULT,
        # [doc = "< discard all non reference"]
        NonReference = ffi::AVDiscard_AVDISCARD_NONREF,
        # [doc = "< discard all bidirectional frames"]
        BiDirectional = ffi::AVDiscard_AVDISCARD_BIDIR,
        # [doc = "< discard all non intra frames"]
        NonIntra = ffi::AVDiscard_AVDISCARD_NONINTRA,
        # [doc = "< discard all frames except keyframes"]
        NonKey = ffi::AVDiscard_AVDISCARD_NONKEY,
        # [doc = "< discard all"]
        All =ffi::AVDiscard_AVDISCARD_ALL
    }
}

pub struct AVStream<'a> {
    base: &'a mut ffi::AVStream,
    phantom: PhantomData<&'a AVFormatContext>,
}

impl<'a> AVStream<'a> {
    fn new(base: &'a mut ffi::AVStream, _: &'a AVFormatContext) -> Self {
        return AVStream { base, phantom: PhantomData };
    }

    // TODO: Just for testing
    pub unsafe fn as_ref(&self) -> &ffi::AVStream {
        self.base
    }
    pub unsafe fn as_mut(&mut self) -> &mut ffi::AVStream {
        self.base
    }

    pub fn index(self: &AVStream<'a>) -> i32 {
        self.base.index
    }

    pub fn time_base(self: &AVStream<'a>) -> Fraction {
        Fraction::new(
            self.base.time_base.num as u32,
            self.base.time_base.den as u32,
        )
    }

    pub fn timestamp(self: &AVStream<'a>, timestamp: i64) -> std::time::Duration {
        std::time::Duration::from_millis(
            1000 *
                timestamp as u64 *
                self.base.time_base.num as u64 /
                self.base.time_base.den as u64
        )
    }

    pub fn duration(self: &AVStream<'a>) -> std::time::Duration {
        self.timestamp(self.base.duration)
    }

    pub fn frame_count(self: &AVStream<'a>) -> i64 {
        self.base.nb_frames
    }

    pub fn discard(self: &AVStream<'a>) -> Option<AVDiscard> {
        AVDiscard::from_i32(self.base.discard)
    }

    pub fn set_discard(self: &mut AVStream<'a>, value: AVDiscard) {
        self.base.discard = value as i32;
    }

    pub fn sample_aspect_ratio(self: &AVStream<'a>) -> Fraction {
        Fraction::new(
            self.base.sample_aspect_ratio.num as u32,
            self.base.sample_aspect_ratio.den as u32,
        )
    }

    pub fn codec_parameters(self: &AVStream<'a>) -> AVCodecParameters {
        AVCodecParameters::new(unsafe { self.base.codecpar.as_mut() }.expect("not null"), self)
    }
}

pub struct AVCodecParameters<'a> {
    base: &'a mut ffi::AVCodecParameters,
    phantom: PhantomData<&'a AVStream<'a>>,
}

impl<'a> AVCodecParameters<'a> {
    fn new(base: &'a mut ffi::AVCodecParameters, _: &'a AVStream) -> Self {
        return AVCodecParameters { base, phantom: PhantomData };
    }

    // TODO: Just for testing
    pub unsafe fn as_ref(&self) -> &ffi::AVCodecParameters {
        self.base
    }
    pub unsafe fn as_mut(&mut self) -> &mut ffi::AVCodecParameters {
        self.base
    }

    pub fn codec_type(self: &AVCodecParameters<'a>) -> AVMediaType {
        AVMediaType::from_i32(self.base.codec_type).unwrap_or(AVMediaType::Unknown)
    }

    pub fn codec_id(self: &AVCodecParameters<'a>) -> Option<AVCodecID> {
        AVCodecID::from_u32(self.base.codec_id)
    }

    pub fn find_decoder(self: &AVCodecParameters<'a>) -> AVCodec {
        AVCodec::new(
            unsafe { ffi::avcodec_find_decoder(self.base.codec_id).as_mut() }.expect("Decoder not found"),
            self,
        )
    }
}

pub struct AVCodec<'a> {
    base: &'a mut ffi::AVCodec,
    phantom: PhantomData<&'a AVCodecParameters<'a>>,
}

impl<'a> AVCodec<'a> {
    fn new(base: &'a mut ffi::AVCodec, _: &'a AVCodecParameters) -> Self {
        return AVCodec { base, phantom: PhantomData };
    }

    // TODO: Just for testing
    pub unsafe fn as_ref(&self) -> &ffi::AVCodec {
        self.base
    }
    pub unsafe fn as_mut(&mut self) -> &mut ffi::AVCodec {
        self.base
    }

    pub fn name(self: &AVCodec<'a>) -> std::string::String {
        String::from(unsafe { std::ffi::CStr::from_ptr(self.base.name) }.to_str().unwrap())
    }
}
