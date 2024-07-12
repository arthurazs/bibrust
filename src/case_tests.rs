#[cfg(test)]
pub mod case1 {
    use std::io::Cursor;
    pub struct Case {
        pub file: Cursor<String>,
        pub expected_entry1: Cursor<Vec<u8>>,
        pub expected_tell1: u64,
        pub expected_entry2: Cursor<Vec<u8>>,
        pub expected_tell2: u64,
        pub expected_entry3: Cursor<Vec<u8>>,
        pub expected_tell3: u64,
        pub expected_entry4: Cursor<Vec<u8>>,
        pub expected_tell4: u64,
    }

    impl Case {
        pub fn new() -> [Self; 3] {
            let acm = Case {
                file: Cursor::new(String::from(
                    r"
@article{1,
author = {Ahmad, Waqar and Hasan, Osman and Tahar, Sofi\`{e}ne},
title = {Formal reliability and failure analysis of ethernet based communication networks in a smart grid substation},
year = {2020},
issue_date = {Feb 2020},
publisher = {Springer-Verlag},
address = {Berlin, Heidelberg},
volume = {32},
number = {1},
issn = {0934-5043},
url = {https://doi.org/10.1007/s00165-019-00503-1},
doi = {10.1007/s00165-019-00503-1},
journal = {Form. Asp. Comput.},
month = {feb},
pages = {71–111},
numpages = {41},
keywords = {Theorem proving, Higher-order logic, Fault tree, Reliability block diagrams, Smart grid}
}

@article{2,
author = {Formby, David and Walid, Anwar and Beyah, Raheem},
title = {A Case Study in Power Substation Network Dynamics},
year = {2017},
issue_date = {June 2017},
publisher = {Association for Computing Machinery},
address = {New York, NY, USA},
volume = {1},
number = {1},
url = {https://doi.org/10.1145/3084456},
doi = {10.1145/3084456},
journal = {Proc. ACM Meas. Anal. Comput. Syst.},
month = {jun},
articleno = {19},
numpages = {24},
keywords = {scada, power grid, network characterization}
}
",
                )),
                expected_entry1: Cursor::new(Vec::from([
                    10, 64, 97, 114, 116, 105, 99, 108, 101, 123, 49, 44, 10, 97, 117, 116, 104,
                    111, 114, 32, 61, 32, 123, 65, 104, 109, 97, 100, 44, 32, 87, 97, 113, 97, 114,
                    32, 97, 110, 100, 32, 72, 97, 115, 97, 110, 44, 32, 79, 115, 109, 97, 110, 32,
                    97, 110, 100, 32, 84, 97, 104, 97, 114, 44, 32, 83, 111, 102, 105, 92, 96, 123,
                    101, 125, 110, 101, 125, 44, 10, 116, 105, 116, 108, 101, 32, 61, 32, 123, 70,
                    111, 114, 109, 97, 108, 32, 114, 101, 108, 105, 97, 98, 105, 108, 105, 116,
                    121, 32, 97, 110, 100, 32, 102, 97, 105, 108, 117, 114, 101, 32, 97, 110, 97,
                    108, 121, 115, 105, 115, 32, 111, 102, 32, 101, 116, 104, 101, 114, 110, 101,
                    116, 32, 98, 97, 115, 101, 100, 32, 99, 111, 109, 109, 117, 110, 105, 99, 97,
                    116, 105, 111, 110, 32, 110, 101, 116, 119, 111, 114, 107, 115, 32, 105, 110,
                    32, 97, 32, 115, 109, 97, 114, 116, 32, 103, 114, 105, 100, 32, 115, 117, 98,
                    115, 116, 97, 116, 105, 111, 110, 125, 44, 10, 121, 101, 97, 114, 32, 61, 32,
                    123, 50, 48, 50, 48, 125, 44, 10, 105, 115, 115, 117, 101, 95, 100, 97, 116,
                    101, 32, 61, 32, 123, 70, 101, 98, 32, 50, 48, 50, 48, 125, 44, 10, 112, 117,
                    98, 108, 105, 115, 104, 101, 114, 32, 61, 32, 123, 83, 112, 114, 105, 110, 103,
                    101, 114, 45, 86, 101, 114, 108, 97, 103, 125, 44, 10, 97, 100, 100, 114, 101,
                    115, 115, 32, 61, 32, 123, 66, 101, 114, 108, 105, 110, 44, 32, 72, 101, 105,
                    100, 101, 108, 98, 101, 114, 103, 125, 44, 10, 118, 111, 108, 117, 109, 101,
                    32, 61, 32, 123, 51, 50, 125, 44, 10, 110, 117, 109, 98, 101, 114, 32, 61, 32,
                    123, 49, 125, 44, 10, 105, 115, 115, 110, 32, 61, 32, 123, 48, 57, 51, 52, 45,
                    53, 48, 52, 51, 125, 44, 10, 117, 114, 108, 32, 61, 32, 123, 104, 116, 116,
                    112, 115, 58, 47, 47, 100, 111, 105, 46, 111, 114, 103, 47, 49, 48, 46, 49, 48,
                    48, 55, 47, 115, 48, 48, 49, 54, 53, 45, 48, 49, 57, 45, 48, 48, 53, 48, 51,
                    45, 49, 125, 44, 10, 100, 111, 105, 32, 61, 32, 123, 49, 48, 46, 49, 48, 48,
                    55, 47, 115, 48, 48, 49, 54, 53, 45, 48, 49, 57, 45, 48, 48, 53, 48, 51, 45,
                    49, 125, 44, 10, 106, 111, 117, 114, 110, 97, 108, 32, 61, 32, 123, 70, 111,
                    114, 109, 46, 32, 65, 115, 112, 46, 32, 67, 111, 109, 112, 117, 116, 46, 125,
                    44, 10, 109, 111, 110, 116, 104, 32, 61, 32, 123, 102, 101, 98, 125, 44, 10,
                    112, 97, 103, 101, 115, 32, 61, 32, 123, 55, 49, 226, 128, 147, 49, 49, 49,
                    125, 44, 10, 110, 117, 109, 112, 97, 103, 101, 115, 32, 61, 32, 123, 52, 49,
                    125, 44, 10, 107, 101, 121, 119, 111, 114, 100, 115, 32, 61, 32, 123, 84, 104,
                    101, 111, 114, 101, 109, 32, 112, 114, 111, 118, 105, 110, 103, 44, 32, 72,
                    105, 103, 104, 101, 114, 45, 111, 114, 100, 101, 114, 32, 108, 111, 103, 105,
                    99, 44, 32, 70, 97, 117, 108, 116, 32, 116, 114, 101, 101, 44, 32, 82, 101,
                    108, 105, 97, 98, 105, 108, 105, 116, 121, 32, 98, 108, 111, 99, 107, 32, 100,
                    105, 97, 103, 114, 97, 109, 115, 44, 32, 83, 109, 97, 114, 116, 32, 103, 114,
                    105, 100, 125, 10, 125,
                ])),
                expected_tell1: 623,
                expected_entry2: Cursor::new(Vec::from([
                    10, 10, 64, 97, 114, 116, 105, 99, 108, 101, 123, 50, 44, 10, 97, 117, 116,
                    104, 111, 114, 32, 61, 32, 123, 70, 111, 114, 109, 98, 121, 44, 32, 68, 97,
                    118, 105, 100, 32, 97, 110, 100, 32, 87, 97, 108, 105, 100, 44, 32, 65, 110,
                    119, 97, 114, 32, 97, 110, 100, 32, 66, 101, 121, 97, 104, 44, 32, 82, 97, 104,
                    101, 101, 109, 125, 44, 10, 116, 105, 116, 108, 101, 32, 61, 32, 123, 65, 32,
                    67, 97, 115, 101, 32, 83, 116, 117, 100, 121, 32, 105, 110, 32, 80, 111, 119,
                    101, 114, 32, 83, 117, 98, 115, 116, 97, 116, 105, 111, 110, 32, 78, 101, 116,
                    119, 111, 114, 107, 32, 68, 121, 110, 97, 109, 105, 99, 115, 125, 44, 10, 121,
                    101, 97, 114, 32, 61, 32, 123, 50, 48, 49, 55, 125, 44, 10, 105, 115, 115, 117,
                    101, 95, 100, 97, 116, 101, 32, 61, 32, 123, 74, 117, 110, 101, 32, 50, 48, 49,
                    55, 125, 44, 10, 112, 117, 98, 108, 105, 115, 104, 101, 114, 32, 61, 32, 123,
                    65, 115, 115, 111, 99, 105, 97, 116, 105, 111, 110, 32, 102, 111, 114, 32, 67,
                    111, 109, 112, 117, 116, 105, 110, 103, 32, 77, 97, 99, 104, 105, 110, 101,
                    114, 121, 125, 44, 10, 97, 100, 100, 114, 101, 115, 115, 32, 61, 32, 123, 78,
                    101, 119, 32, 89, 111, 114, 107, 44, 32, 78, 89, 44, 32, 85, 83, 65, 125, 44,
                    10, 118, 111, 108, 117, 109, 101, 32, 61, 32, 123, 49, 125, 44, 10, 110, 117,
                    109, 98, 101, 114, 32, 61, 32, 123, 49, 125, 44, 10, 117, 114, 108, 32, 61, 32,
                    123, 104, 116, 116, 112, 115, 58, 47, 47, 100, 111, 105, 46, 111, 114, 103, 47,
                    49, 48, 46, 49, 49, 52, 53, 47, 51, 48, 56, 52, 52, 53, 54, 125, 44, 10, 100,
                    111, 105, 32, 61, 32, 123, 49, 48, 46, 49, 49, 52, 53, 47, 51, 48, 56, 52, 52,
                    53, 54, 125, 44, 10, 106, 111, 117, 114, 110, 97, 108, 32, 61, 32, 123, 80,
                    114, 111, 99, 46, 32, 65, 67, 77, 32, 77, 101, 97, 115, 46, 32, 65, 110, 97,
                    108, 46, 32, 67, 111, 109, 112, 117, 116, 46, 32, 83, 121, 115, 116, 46, 125,
                    44, 10, 109, 111, 110, 116, 104, 32, 61, 32, 123, 106, 117, 110, 125, 44, 10,
                    97, 114, 116, 105, 99, 108, 101, 110, 111, 32, 61, 32, 123, 49, 57, 125, 44,
                    10, 110, 117, 109, 112, 97, 103, 101, 115, 32, 61, 32, 123, 50, 52, 125, 44,
                    10, 107, 101, 121, 119, 111, 114, 100, 115, 32, 61, 32, 123, 115, 99, 97, 100,
                    97, 44, 32, 112, 111, 119, 101, 114, 32, 103, 114, 105, 100, 44, 32, 110, 101,
                    116, 119, 111, 114, 107, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114, 105, 122,
                    97, 116, 105, 111, 110, 125, 10, 125,
                ])),
                expected_tell2: 1133,
                expected_entry3: Cursor::new(Vec::from([10])),
                expected_tell3: 1133 + 1,
                expected_entry4: Cursor::new(Vec::new()),
                expected_tell4: 1133 + 1,
            };

            let ieee = Case {
                file: Cursor::new(String::from(
                    "
@ARTICLE{1,
  author={Wang, Wenlong and Liu, Minghui and Zhao, Xicai and Yang, Gui},
  journal={Journal of Modern Power Systems and Clean Energy}, 
  title={Shared-network scheme of SMV and GOOSE in smart substation}, 
  year={2014},
  volume={2},
  number={4},
  pages={438-443},
  doi={10.1007/s40565-014-0073-z},
  ISSN={2196-5420},
  month={December},}@ARTICLE{2,
  author={Ali, Ikbal and Hussain, S. M. Suhail and Tak, Ashok and Ustun, Taha Selim},
  journal={IEEE Transactions on Industry Applications}, 
  title={Communication Modeling for Differential Protection in IEC-61850-Based Substations}, 
  year={2018},
  volume={54},
  number={1},
  pages={135-142},
  doi={10.1109/TIA.2017.2740301},
  ISSN={1939-9367},
  month={Jan},}
",
                )),
                expected_entry1: Cursor::new(Vec::from([
                    10, 64, 65, 82, 84, 73, 67, 76, 69, 123, 49, 44, 10, 32, 32, 97, 117, 116, 104,
                    111, 114, 61, 123, 87, 97, 110, 103, 44, 32, 87, 101, 110, 108, 111, 110, 103,
                    32, 97, 110, 100, 32, 76, 105, 117, 44, 32, 77, 105, 110, 103, 104, 117, 105,
                    32, 97, 110, 100, 32, 90, 104, 97, 111, 44, 32, 88, 105, 99, 97, 105, 32, 97,
                    110, 100, 32, 89, 97, 110, 103, 44, 32, 71, 117, 105, 125, 44, 10, 32, 32, 106,
                    111, 117, 114, 110, 97, 108, 61, 123, 74, 111, 117, 114, 110, 97, 108, 32, 111,
                    102, 32, 77, 111, 100, 101, 114, 110, 32, 80, 111, 119, 101, 114, 32, 83, 121,
                    115, 116, 101, 109, 115, 32, 97, 110, 100, 32, 67, 108, 101, 97, 110, 32, 69,
                    110, 101, 114, 103, 121, 125, 44, 32, 10, 32, 32, 116, 105, 116, 108, 101, 61,
                    123, 83, 104, 97, 114, 101, 100, 45, 110, 101, 116, 119, 111, 114, 107, 32,
                    115, 99, 104, 101, 109, 101, 32, 111, 102, 32, 83, 77, 86, 32, 97, 110, 100,
                    32, 71, 79, 79, 83, 69, 32, 105, 110, 32, 115, 109, 97, 114, 116, 32, 115, 117,
                    98, 115, 116, 97, 116, 105, 111, 110, 125, 44, 32, 10, 32, 32, 121, 101, 97,
                    114, 61, 123, 50, 48, 49, 52, 125, 44, 10, 32, 32, 118, 111, 108, 117, 109,
                    101, 61, 123, 50, 125, 44, 10, 32, 32, 110, 117, 109, 98, 101, 114, 61, 123,
                    52, 125, 44, 10, 32, 32, 112, 97, 103, 101, 115, 61, 123, 52, 51, 56, 45, 52,
                    52, 51, 125, 44, 10, 32, 32, 100, 111, 105, 61, 123, 49, 48, 46, 49, 48, 48,
                    55, 47, 115, 52, 48, 53, 54, 53, 45, 48, 49, 52, 45, 48, 48, 55, 51, 45, 122,
                    125, 44, 10, 32, 32, 73, 83, 83, 78, 61, 123, 50, 49, 57, 54, 45, 53, 52, 50,
                    48, 125, 44, 10, 32, 32, 109, 111, 110, 116, 104, 61, 123, 68, 101, 99, 101,
                    109, 98, 101, 114, 125, 44, 125,
                ])),
                expected_tell1: 357,
                expected_entry2: Cursor::new(Vec::from([
                    64, 65, 82, 84, 73, 67, 76, 69, 123, 50, 44, 10, 32, 32, 97, 117, 116, 104,
                    111, 114, 61, 123, 65, 108, 105, 44, 32, 73, 107, 98, 97, 108, 32, 97, 110,
                    100, 32, 72, 117, 115, 115, 97, 105, 110, 44, 32, 83, 46, 32, 77, 46, 32, 83,
                    117, 104, 97, 105, 108, 32, 97, 110, 100, 32, 84, 97, 107, 44, 32, 65, 115,
                    104, 111, 107, 32, 97, 110, 100, 32, 85, 115, 116, 117, 110, 44, 32, 84, 97,
                    104, 97, 32, 83, 101, 108, 105, 109, 125, 44, 10, 32, 32, 106, 111, 117, 114,
                    110, 97, 108, 61, 123, 73, 69, 69, 69, 32, 84, 114, 97, 110, 115, 97, 99, 116,
                    105, 111, 110, 115, 32, 111, 110, 32, 73, 110, 100, 117, 115, 116, 114, 121,
                    32, 65, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110, 115, 125, 44, 32, 10,
                    32, 32, 116, 105, 116, 108, 101, 61, 123, 67, 111, 109, 109, 117, 110, 105, 99,
                    97, 116, 105, 111, 110, 32, 77, 111, 100, 101, 108, 105, 110, 103, 32, 102,
                    111, 114, 32, 68, 105, 102, 102, 101, 114, 101, 110, 116, 105, 97, 108, 32, 80,
                    114, 111, 116, 101, 99, 116, 105, 111, 110, 32, 105, 110, 32, 73, 69, 67, 45,
                    54, 49, 56, 53, 48, 45, 66, 97, 115, 101, 100, 32, 83, 117, 98, 115, 116, 97,
                    116, 105, 111, 110, 115, 125, 44, 32, 10, 32, 32, 121, 101, 97, 114, 61, 123,
                    50, 48, 49, 56, 125, 44, 10, 32, 32, 118, 111, 108, 117, 109, 101, 61, 123, 53,
                    52, 125, 44, 10, 32, 32, 110, 117, 109, 98, 101, 114, 61, 123, 49, 125, 44, 10,
                    32, 32, 112, 97, 103, 101, 115, 61, 123, 49, 51, 53, 45, 49, 52, 50, 125, 44,
                    10, 32, 32, 100, 111, 105, 61, 123, 49, 48, 46, 49, 49, 48, 57, 47, 84, 73, 65,
                    46, 50, 48, 49, 55, 46, 50, 55, 52, 48, 51, 48, 49, 125, 44, 10, 32, 32, 73,
                    83, 83, 78, 61, 123, 49, 57, 51, 57, 45, 57, 51, 54, 55, 125, 44, 10, 32, 32,
                    109, 111, 110, 116, 104, 61, 123, 74, 97, 110, 125, 44, 125,
                ])),
                expected_tell2: 738,
                expected_entry3: Cursor::new(Vec::from([10])),
                expected_tell3: 738 + 1,
                expected_entry4: Cursor::new(Vec::new()),
                expected_tell4: 738 + 1,
            };
            let science_directory = Case {
                file: Cursor::new(String::from("
@article{1,
title = {Research and implementation of virtual circuit test tool for smart substations},
journal = {Procedia Computer Science},
volume = {183},
pages = {197-204},
year = {2021},
note = {Proceedings of the 10th International Conference of Information and Communication Technology},
issn = {1877-0509},
doi = {https://doi.org/10.1016/j.procs.2021.02.050},
url = {https://www.sciencedirect.com/science/article/pii/S1877050921005159},
author = {Jin Wang and Zengkai Wang},
keywords = {Smart substation, IEC61850, virtual circuit},
}
@article{2,
title = {Comparative analysis of the DAQ cards-based and the IEC 61850-based real time simulations in the matlab/simulink environment for power system protections},
journal = {Electric Power Systems Research},
volume = {192},
pages = {107000},
year = {2021},
issn = {0378-7796},
doi = {https://doi.org/10.1016/j.epsr.2020.107000},
url = {https://www.sciencedirect.com/science/article/pii/S0378779620307987},
author = {M. Krakowski and K. Kurek and L. Nogal},
keywords = {Hardware-in-the-loop, Real time simulations, DAQ Cards, IEC 61850, Real time Linux},
}
",
                )),
                expected_entry1: Cursor::new(Vec::from([
                    10, 64, 97, 114, 116, 105, 99, 108, 101, 123, 49, 44, 10, 116, 105, 116, 108,
                    101, 32, 61, 32, 123, 82, 101, 115, 101, 97, 114, 99, 104, 32, 97, 110, 100,
                    32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32,
                    111, 102, 32, 118, 105, 114, 116, 117, 97, 108, 32, 99, 105, 114, 99, 117, 105,
                    116, 32, 116, 101, 115, 116, 32, 116, 111, 111, 108, 32, 102, 111, 114, 32,
                    115, 109, 97, 114, 116, 32, 115, 117, 98, 115, 116, 97, 116, 105, 111, 110,
                    115, 125, 44, 10, 106, 111, 117, 114, 110, 97, 108, 32, 61, 32, 123, 80, 114,
                    111, 99, 101, 100, 105, 97, 32, 67, 111, 109, 112, 117, 116, 101, 114, 32, 83,
                    99, 105, 101, 110, 99, 101, 125, 44, 10, 118, 111, 108, 117, 109, 101, 32, 61,
                    32, 123, 49, 56, 51, 125, 44, 10, 112, 97, 103, 101, 115, 32, 61, 32, 123, 49,
                    57, 55, 45, 50, 48, 52, 125, 44, 10, 121, 101, 97, 114, 32, 61, 32, 123, 50,
                    48, 50, 49, 125, 44, 10, 110, 111, 116, 101, 32, 61, 32, 123, 80, 114, 111, 99,
                    101, 101, 100, 105, 110, 103, 115, 32, 111, 102, 32, 116, 104, 101, 32, 49, 48,
                    116, 104, 32, 73, 110, 116, 101, 114, 110, 97, 116, 105, 111, 110, 97, 108, 32,
                    67, 111, 110, 102, 101, 114, 101, 110, 99, 101, 32, 111, 102, 32, 73, 110, 102,
                    111, 114, 109, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 67, 111, 109, 109,
                    117, 110, 105, 99, 97, 116, 105, 111, 110, 32, 84, 101, 99, 104, 110, 111, 108,
                    111, 103, 121, 125, 44, 10, 105, 115, 115, 110, 32, 61, 32, 123, 49, 56, 55,
                    55, 45, 48, 53, 48, 57, 125, 44, 10, 100, 111, 105, 32, 61, 32, 123, 104, 116,
                    116, 112, 115, 58, 47, 47, 100, 111, 105, 46, 111, 114, 103, 47, 49, 48, 46,
                    49, 48, 49, 54, 47, 106, 46, 112, 114, 111, 99, 115, 46, 50, 48, 50, 49, 46,
                    48, 50, 46, 48, 53, 48, 125, 44, 10, 117, 114, 108, 32, 61, 32, 123, 104, 116,
                    116, 112, 115, 58, 47, 47, 119, 119, 119, 46, 115, 99, 105, 101, 110, 99, 101,
                    100, 105, 114, 101, 99, 116, 46, 99, 111, 109, 47, 115, 99, 105, 101, 110, 99,
                    101, 47, 97, 114, 116, 105, 99, 108, 101, 47, 112, 105, 105, 47, 83, 49, 56,
                    55, 55, 48, 53, 48, 57, 50, 49, 48, 48, 53, 49, 53, 57, 125, 44, 10, 97, 117,
                    116, 104, 111, 114, 32, 61, 32, 123, 74, 105, 110, 32, 87, 97, 110, 103, 32,
                    97, 110, 100, 32, 90, 101, 110, 103, 107, 97, 105, 32, 87, 97, 110, 103, 125,
                    44, 10, 107, 101, 121, 119, 111, 114, 100, 115, 32, 61, 32, 123, 83, 109, 97,
                    114, 116, 32, 115, 117, 98, 115, 116, 97, 116, 105, 111, 110, 44, 32, 73, 69,
                    67, 54, 49, 56, 53, 48, 44, 32, 118, 105, 114, 116, 117, 97, 108, 32, 99, 105,
                    114, 99, 117, 105, 116, 125, 44, 10, 125,
                ])),
                expected_tell1: 542,
                expected_entry2: Cursor::new(Vec::from([
                    10, 64, 97, 114, 116, 105, 99, 108, 101, 123, 50, 44, 10, 116, 105, 116, 108,
                    101, 32, 61, 32, 123, 67, 111, 109, 112, 97, 114, 97, 116, 105, 118, 101, 32,
                    97, 110, 97, 108, 121, 115, 105, 115, 32, 111, 102, 32, 116, 104, 101, 32, 68,
                    65, 81, 32, 99, 97, 114, 100, 115, 45, 98, 97, 115, 101, 100, 32, 97, 110, 100,
                    32, 116, 104, 101, 32, 73, 69, 67, 32, 54, 49, 56, 53, 48, 45, 98, 97, 115,
                    101, 100, 32, 114, 101, 97, 108, 32, 116, 105, 109, 101, 32, 115, 105, 109,
                    117, 108, 97, 116, 105, 111, 110, 115, 32, 105, 110, 32, 116, 104, 101, 32,
                    109, 97, 116, 108, 97, 98, 47, 115, 105, 109, 117, 108, 105, 110, 107, 32, 101,
                    110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 32, 102, 111, 114, 32, 112,
                    111, 119, 101, 114, 32, 115, 121, 115, 116, 101, 109, 32, 112, 114, 111, 116,
                    101, 99, 116, 105, 111, 110, 115, 125, 44, 10, 106, 111, 117, 114, 110, 97,
                    108, 32, 61, 32, 123, 69, 108, 101, 99, 116, 114, 105, 99, 32, 80, 111, 119,
                    101, 114, 32, 83, 121, 115, 116, 101, 109, 115, 32, 82, 101, 115, 101, 97, 114,
                    99, 104, 125, 44, 10, 118, 111, 108, 117, 109, 101, 32, 61, 32, 123, 49, 57,
                    50, 125, 44, 10, 112, 97, 103, 101, 115, 32, 61, 32, 123, 49, 48, 55, 48, 48,
                    48, 125, 44, 10, 121, 101, 97, 114, 32, 61, 32, 123, 50, 48, 50, 49, 125, 44,
                    10, 105, 115, 115, 110, 32, 61, 32, 123, 48, 51, 55, 56, 45, 55, 55, 57, 54,
                    125, 44, 10, 100, 111, 105, 32, 61, 32, 123, 104, 116, 116, 112, 115, 58, 47,
                    47, 100, 111, 105, 46, 111, 114, 103, 47, 49, 48, 46, 49, 48, 49, 54, 47, 106,
                    46, 101, 112, 115, 114, 46, 50, 48, 50, 48, 46, 49, 48, 55, 48, 48, 48, 125,
                    44, 10, 117, 114, 108, 32, 61, 32, 123, 104, 116, 116, 112, 115, 58, 47, 47,
                    119, 119, 119, 46, 115, 99, 105, 101, 110, 99, 101, 100, 105, 114, 101, 99,
                    116, 46, 99, 111, 109, 47, 115, 99, 105, 101, 110, 99, 101, 47, 97, 114, 116,
                    105, 99, 108, 101, 47, 112, 105, 105, 47, 83, 48, 51, 55, 56, 55, 55, 57, 54,
                    50, 48, 51, 48, 55, 57, 56, 55, 125, 44, 10, 97, 117, 116, 104, 111, 114, 32,
                    61, 32, 123, 77, 46, 32, 75, 114, 97, 107, 111, 119, 115, 107, 105, 32, 97,
                    110, 100, 32, 75, 46, 32, 75, 117, 114, 101, 107, 32, 97, 110, 100, 32, 76, 46,
                    32, 78, 111, 103, 97, 108, 125, 44, 10, 107, 101, 121, 119, 111, 114, 100, 115,
                    32, 61, 32, 123, 72, 97, 114, 100, 119, 97, 114, 101, 45, 105, 110, 45, 116,
                    104, 101, 45, 108, 111, 111, 112, 44, 32, 82, 101, 97, 108, 32, 116, 105, 109,
                    101, 32, 115, 105, 109, 117, 108, 97, 116, 105, 111, 110, 115, 44, 32, 68, 65,
                    81, 32, 67, 97, 114, 100, 115, 44, 32, 73, 69, 67, 32, 54, 49, 56, 53, 48, 44,
                    32, 82, 101, 97, 108, 32, 116, 105, 109, 101, 32, 76, 105, 110, 117, 120, 125,
                    44, 10, 125,
                ])),
                expected_tell2: 1112,
                expected_entry3: Cursor::new(Vec::from([10])),
                expected_tell3: 1112 + 1,
                expected_entry4: Cursor::new(Vec::from([])),
                expected_tell4: 1112 + 1,
            };
            [acm, ieee, science_directory]
        }
    }
}
