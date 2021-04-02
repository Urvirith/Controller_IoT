//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                CONTROLLER - IOT - CANOPEN                                                  ///
///                                DESIGNED BY JACOB MUSSLER                                                   ///
///                                17-MAR-2020 PROOF OF PRINCIPLE FOR RUST LANGUAGE                            ///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///                                Communication Objects                                                       ///
///                                PROCESS DATA OBJECTS (PDO)                                                  ///
///                                SERVICE DATA OBJECTS (SDO)                                                  ///
///                                SPECIAL FUNCTION OBJECTS (SFO)                                              ///
///                                NETWORK MANAGEMENT OBJECTS (NMO)                                            ///
///                                                                                                            ///
///                                CANOpen Frame                                                               ///
///                                CAN-ID  | RTR   | DATA LENGTH | DATA                                        ///
///                                11 BITS | 1 BIT | 4 - BITS    | 0 - 8 BYTES                                 ///
///                                CAN FRAME                                                                   /// 
///                                CAN-ID  | RTR   | Reserved | DATA LENGTH | DATA                             ///
///                                11 BITS | 1 BIT | 2 - BITS | 4 - BITS    | 0 - 8 BYTES                      ///
///                                HANDLED AUTOMATICALLY VIA CAN_TIxR REGISTER PG.1506 & TDTxR PG.1507         ///
///                                                                                                            ///
///                                CAN-ID                                                                      ///
///                                FUNCTION CODE | NODE ID                                                     ///
///                                4 BITS        | 7 BITS                                                      ///
///                                CANOpen uses Little Endian                                                  ///                               
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

//Development of the CANOPEN / Powerlink -> EtherCAT also uses CANOPEN Protocols
pub mod handler;

