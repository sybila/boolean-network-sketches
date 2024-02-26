pub mod reachability_one_hole;
pub mod reachability_two_holes;
pub mod utils;

pub const MODEL_20V: &str = r"
$v_cMYC:(v_Akt1 | (v_ERa | v_MEK1))
v_Akt1 -> v_cMYC
v_ERa -> v_cMYC
v_MEK1 -> v_cMYC
$v_p27:(!v_Akt1 & (!v_CDK2 & (!v_CDK4 & (v_ERa & !v_cMYC))))
v_cMYC -| v_p27
v_CDK2 -| v_p27
v_Akt1 -| v_p27
v_CDK4 -| v_p27
v_ERa -> v_p27
$v_CDK2:(v_CycE1 & (!v_p21 & !v_p27))
v_p27 -| v_CDK2
v_CycE1 -> v_CDK2
v_p21 -| v_CDK2
$v_Akt1:(v_ErbB1 | (v_ErbB1_2 | (v_ErbB1_3 | (v_ErbB2_3 | v_IGF1R))))
v_ErbB2_3 -> v_Akt1
v_ErbB1_2 -> v_Akt1
v_ErbB1_3 -> v_Akt1
v_IGF1R -> v_Akt1
v_ErbB1 -> v_Akt1
$v_CDK4:(v_CycD1 & (!v_p21 & !v_p27))
v_CycD1 -> v_CDK4
v_p27 -| v_CDK4
v_p21 -| v_CDK4
$v_ERa:(v_Akt1 | v_MEK1)
v_Akt1 -> v_ERa
v_MEK1 -> v_ERa
$v_CycE1:v_cMYC
v_cMYC -> v_CycE1
$v_p21:(!v_Akt1 & (!v_CDK4 & (v_ERa & !v_cMYC)))
v_cMYC -| v_p21
v_Akt1 -| v_p21
v_CDK4 -| v_p21
v_ERa -> v_p21
$v_ErbB2_3:(v_ErbB2 & v_ErbB3)
v_ErbB3 -> v_ErbB2_3
v_ErbB2 -> v_ErbB2_3
$v_ErbB1_2:(v_ErbB1 & v_ErbB2)
v_ErbB1 -> v_ErbB1_2
v_ErbB2 -> v_ErbB1_2
$v_ErbB1_3:(v_ErbB1 & v_ErbB3)
v_ErbB3 -> v_ErbB1_3
v_ErbB1 -> v_ErbB1_3
$v_IGF1R:((v_Akt1 & !v_ErbB2_3) | (!v_Akt1 & (v_ERa & !v_ErbB2_3)))
v_ErbB2_3 -| v_IGF1R
v_Akt1 -> v_IGF1R
v_ERa -> v_IGF1R
$v_ErbB1:v_EGF
v_EGF -> v_ErbB1
$v_MEK1:(v_ErbB1 | (v_ErbB1_2 | (v_ErbB1_3 | (v_ErbB2_3 | v_IGF1R))))
v_ErbB2_3 -> v_MEK1
v_ErbB1_2 -> v_MEK1
v_ErbB1_3 -> v_MEK1
v_ErbB1 -> v_MEK1
v_IGF1R -> v_MEK1
$v_EGF:true
$v_ErbB3:v_EGF
v_EGF -> v_ErbB3
$v_CycD1:((v_Akt1 & (v_ERa & v_cMYC)) | (!v_Akt1 & (v_ERa & (v_MEK1 & v_cMYC))))
v_cMYC -> v_CycD1
v_Akt1 -> v_CycD1
v_ERa -> v_CycD1
v_MEK1 -> v_CycD1
$v_ErbB2:v_EGF
v_EGF -> v_ErbB2
$v_CDK6:v_CycD1
v_CycD1 -> v_CDK6
$v_pRB:(v_CDK4 & v_CDK6)
v_CDK6 -> v_pRB
v_CDK2 ->? v_pRB
v_CDK4 -> v_pRB
";

pub const MODEL_53V: &str = r"
$v_DCII_Bacterium:v_DCI_Bacterium
v_DCI_Bacterium -> v_DCII_Bacterium
$v_T0:(v_DCII_Bacterium | v_DCII_TRetortaeformis)
v_DCII_Bacterium -> v_T0
v_DCII_TRetortaeformis -> v_T0
$v_DCII_TRetortaeformis:v_DCI_TRetortaeformis
v_DCI_TRetortaeformis -> v_DCII_TRetortaeformis
$v_Th2II_TRetortaeformis:(v_DCII_TRetortaeformis & (!v_IL12II & v_T0))
v_T0 -> v_Th2II_TRetortaeformis
v_DCII_TRetortaeformis -> v_Th2II_TRetortaeformis
v_IL12II -| v_Th2II_TRetortaeformis
$v_IL12II:((v_DCII_Bacterium & (!v_IL4II & v_T0)) | (!v_DCII_Bacterium & (v_DCII_TRetortaeformis & (!v_IL4II & v_T0))))
v_T0 -> v_IL12II
v_DCII_Bacterium -> v_IL12II
v_DCII_TRetortaeformis -> v_IL12II
v_IL4II -| v_IL12II
$v_DCI_TRetortaeformis:v_PIC
v_PIC -> v_DCI_TRetortaeformis
$v_IL4II:((v_DCII_Bacterium & (v_EL2 | (!v_IFNgI & (!v_IL12II & (v_T0 | (v_Th2II_Bacterium | v_Th2II_TRetortaeformis)))))) | (!v_DCII_Bacterium & ((v_DCII_TRetortaeformis & (v_EL2 | (!v_IFNgI & (!v_IL12II & (v_T0 | (v_Th2II_Bacterium | v_Th2II_TRetortaeformis)))))) | (!v_DCII_TRetortaeformis & (v_EL2 | (!v_IFNgI & (!v_IL12II & (v_Th2II_Bacterium | v_Th2II_TRetortaeformis))))))))
v_DCII_Bacterium -> v_IL4II
v_T0 -> v_IL4II
v_Th2II_Bacterium -> v_IL4II
v_DCII_TRetortaeformis -> v_IL4II
v_Th2II_TRetortaeformis -> v_IL4II
v_EL2 -> v_IL4II
v_IL12II -| v_IL4II
v_IFNgI -| v_IL4II
$v_Th2II_Bacterium:(v_DCII_Bacterium & (!v_IL12II & v_T0))
v_DCII_Bacterium -> v_Th2II_Bacterium
v_T0 -> v_Th2II_Bacterium
v_IL12II -| v_Th2II_Bacterium
$v_EL2:((v_IL13 & v_IL5) | (!v_IL13 & (v_IL5 & v_IgE)))
v_IL13 -> v_EL2
v_IL5 -> v_EL2
v_IgE -> v_EL2
$v_IFNgI:(v_DCI_TRetortaeformis | (v_IFNg_Bacterium | v_Th1I_TRetortaeformis))
v_Th1I_TRetortaeformis -> v_IFNgI
v_IFNg_Bacterium -> v_IFNgI
v_DCI_TRetortaeformis -> v_IFNgI
$v_IL4I:v_IL4II
v_IL4II -> v_IL4I
$v_BC_TRetortaeformis:(v_BC_TRetortaeformis | v_T0)
v_T0 -> v_BC_TRetortaeformis
v_BC_TRetortaeformis -> v_BC_TRetortaeformis
$v_IS:false
$v_IgA_TRetortaeformis:(v_BC_TRetortaeformis & v_IS)
v_IS -> v_IgA_TRetortaeformis
v_BC_TRetortaeformis -> v_IgA_TRetortaeformis
$v_AP:((v_AgAb_Bacterium & (v_Bb & (v_MPI_Bacterium & v_Th1I_Bacterium))) | (!v_AgAb_Bacterium & (v_Bb & (v_Cb & (v_IgG_Bacterium & (v_MPI_Bacterium & v_Th1I_Bacterium))))))
v_Cb -> v_AP
v_Th1I_Bacterium -> v_AP
v_IgG_Bacterium -> v_AP
v_AgAb_Bacterium -> v_AP
v_MPI_Bacterium -> v_AP
v_Bb -> v_AP
$v_PH:(v_AP & v_Bb)
v_AP -> v_PH
v_Bb -> v_PH
$v_Bb:(v_Bb & !v_PH)
v_PH -| v_Bb
v_Bb -> v_Bb
$v_EC_Bacterium:v_Bb
v_Bb -> v_EC_Bacterium
$v_Th2I_Bacterium:v_Th2II_Bacterium
v_Th2II_Bacterium -> v_Th2I_Bacterium
$v_IL13:((v_EL & (v_EL2 | (v_IS | (v_Th2I_Bacterium | v_Th2I_TRetortaeformis)))) | (!v_EL & (v_EL2 | (v_Th2I_Bacterium | v_Th2I_TRetortaeformis))))
v_Th2I_Bacterium -> v_IL13
v_Th2I_TRetortaeformis -> v_IL13
v_EL2 -> v_IL13
v_IS -> v_IL13
v_EL -> v_IL13
$v_Th2I_TRetortaeformis:v_Th2II_TRetortaeformis
v_Th2II_TRetortaeformis -> v_Th2I_TRetortaeformis
$v_EL:(!v_EL2 & v_IS)
v_EL2 -| v_EL
v_IS -> v_EL
$v_IL5:(v_EL2 | v_Th2II_TRetortaeformis)
v_Th2II_TRetortaeformis -> v_IL5
v_EL2 -> v_IL5
$v_IgE:(v_BC_TRetortaeformis & (v_IL13 | v_IL4II))
v_IL13 -> v_IgE
v_BC_TRetortaeformis -> v_IgE
v_IL4II -> v_IgE
$v_PIC:((v_AD & (!v_IL10I & !v_IgA_TRetortaeformis)) | (!v_AD & ((v_AP & (!v_IL10I & !v_IgA_TRetortaeformis)) | (!v_AP & ((v_EC_Bacterium & (!v_IL10I & !v_IgA_TRetortaeformis)) | (!v_EC_Bacterium & (v_EC_TRetortaeformis & (!v_IL10I & !v_IgA_TRetortaeformis))))))))
v_AD -> v_PIC
v_IL10I -| v_PIC
v_IgA_TRetortaeformis -| v_PIC
v_EC_Bacterium -> v_PIC
v_AP -> v_PIC
v_EC_TRetortaeformis -> v_PIC
$v_DCI_Bacterium:(v_Bb & (v_IFNg_Bacterium | v_PIC))
v_IFNg_Bacterium -> v_DCI_Bacterium
v_PIC -> v_DCI_Bacterium
v_Bb -> v_DCI_Bacterium
$v_TrII:(v_DCII_Bacterium & (v_T0 & v_TTSSII))
v_DCII_Bacterium -> v_TrII
v_T0 -> v_TrII
v_TTSSII -> v_TrII
$v_TTSSII:v_TTSSI
v_TTSSI -> v_TTSSII
$v_TTSSI:(v_Bb & (!v_IgA_Bacterium & !v_IgG_Bacterium))
v_IgG_Bacterium -| v_TTSSI
v_IgA_Bacterium -| v_TTSSI
v_Bb -> v_TTSSI
$v_IFNg_Bacterium:(v_DCI_Bacterium | ((v_IL10I_Bacterium & v_MPI_Bacterium) | (!v_IL10I_Bacterium & ((v_IL4I & v_MPI_Bacterium) | (!v_IL4I & (v_MPI_Bacterium | v_Th1I_Bacterium))))))
v_IL4I -| v_IFNg_Bacterium
v_DCI_Bacterium -> v_IFNg_Bacterium
v_Th1I_Bacterium -> v_IFNg_Bacterium
v_IL10I_Bacterium -| v_IFNg_Bacterium
v_MPI_Bacterium -> v_IFNg_Bacterium
$v_Th1I_Bacterium:v_Th1II_Bacterium
v_Th1II_Bacterium -> v_Th1I_Bacterium
$v_IL10I_Bacterium:(v_MPI_Bacterium | ((v_TTSSI & (v_Th2I_Bacterium | v_TrI_Bacterium)) | (!v_TTSSI & v_TrI_Bacterium)))
v_Th2I_Bacterium -> v_IL10I_Bacterium
v_TrI_Bacterium -> v_IL10I_Bacterium
v_MPI_Bacterium -> v_IL10I_Bacterium
v_TTSSI -> v_IL10I_Bacterium
$v_MPI_Bacterium:(v_Bb & (v_IFNg_Bacterium | v_PIC))
v_IFNg_Bacterium -> v_MPI_Bacterium
v_PIC -> v_MPI_Bacterium
v_Bb -> v_MPI_Bacterium
$v_AD:((v_AD & (v_IgG & (!v_MPI_Bacterium & !v_NE_TRetortaeformis))) | (!v_AD & (v_IS & (v_IgG & (!v_MPI_Bacterium & !v_NE_TRetortaeformis)))))
v_AD -> v_AD
v_MPI_Bacterium -| v_AD
v_IS -> v_AD
v_IgG -> v_AD
v_NE_TRetortaeformis -| v_AD
$v_IL10I:(v_IL10I_Bacterium | v_Th2I_TRetortaeformis)
v_IL10I_Bacterium -> v_IL10I
v_Th2I_TRetortaeformis -> v_IL10I
$v_EC_TRetortaeformis:(v_AD | v_IS)
v_AD -> v_EC_TRetortaeformis
v_IS -> v_EC_TRetortaeformis
$v_IFNgII:(v_IFNgI | v_IFNg_Bacterium)
v_IFNg_Bacterium -> v_IFNgII
v_IFNgI -> v_IFNgII
$v_Th1I_TRetortaeformis:v_Th1II_TRetortaeformis
v_Th1II_TRetortaeformis -> v_Th1I_TRetortaeformis
$v_IgG_Bacterium:(v_BC_Bacterium | v_IgG_Bacterium)
v_IgG_Bacterium -> v_IgG_Bacterium
v_BC_Bacterium -> v_IgG_Bacterium
$v_BC_Bacterium:(v_BC_Bacterium | v_T0)
v_T0 -> v_BC_Bacterium
v_BC_Bacterium -> v_BC_Bacterium
$v_IgG:v_BC_TRetortaeformis
v_BC_TRetortaeformis -> v_IgG
$v_NE_TRetortaeformis:((v_AD & ((v_IFNgI & ((v_IL10I & v_PIC) | (!v_IL10I & (!v_IL4I | v_PIC)))) | (!v_IFNgI & v_PIC))) | (!v_AD & (v_IFNgI & (!v_IL10I & !v_IL4I))))
v_AD -> v_NE_TRetortaeformis
v_IL4I -| v_NE_TRetortaeformis
v_IL10I -| v_NE_TRetortaeformis
v_IFNgI -> v_NE_TRetortaeformis
v_PIC -> v_NE_TRetortaeformis
$v_NE_Bacterium:v_PIC
v_PIC -> v_NE_Bacterium
$v_DP:(v_NE_Bacterium & v_TTSSI)
v_NE_Bacterium -> v_DP
v_TTSSI -> v_DP
$v_IgA_Bacterium:((v_BC_Bacterium & v_Bb) | (!v_BC_Bacterium & (v_Bb & v_IgA_Bacterium)))
v_BC_Bacterium -> v_IgA_Bacterium
v_IgA_Bacterium -> v_IgA_Bacterium
v_Bb -> v_IgA_Bacterium
$v_Th1II_Bacterium:(v_DCII_Bacterium & (v_IL12II & v_T0))
v_DCII_Bacterium -> v_Th1II_Bacterium
v_T0 -> v_Th1II_Bacterium
v_IL12II -> v_Th1II_Bacterium
$v_TrI_Bacterium:v_TrII
v_TrII -> v_TrI_Bacterium
$v_Cb:((v_AgAb_Bacterium & ((v_Bb & (v_IgG_Bacterium | !v_Oag)) | (!v_Bb & v_IgG_Bacterium))) | (!v_AgAb_Bacterium & (v_Bb & !v_Oag)))
v_Oag -| v_Cb
v_IgG_Bacterium -> v_Cb
v_AgAb_Bacterium -> v_Cb
v_Bb -> v_Cb
$v_AgAb_Bacterium:(v_Bb & (v_IgA_Bacterium | v_IgG_Bacterium))
v_IgG_Bacterium -> v_AgAb_Bacterium
v_IgA_Bacterium -> v_AgAb_Bacterium
v_Bb -> v_AgAb_Bacterium
$v_Oag:v_Bb
v_Bb -> v_Oag
$v_Th1II_TRetortaeformis:(v_DCII_TRetortaeformis & (v_IL12II & v_T0))
v_T0 -> v_Th1II_TRetortaeformis
v_DCII_TRetortaeformis -> v_Th1II_TRetortaeformis
v_IL12II -> v_Th1II_TRetortaeformis
$v_TEL:(v_EL | v_EL2)
v_EL2 -> v_TEL
v_EL -> v_TEL
$v_TNE:(v_NE_Bacterium | v_NE_TRetortaeformis)
v_NE_Bacterium -> v_TNE
v_NE_TRetortaeformis -> v_TNE
";
