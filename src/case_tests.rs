#[cfg(test)]
pub mod cases {
    const EXPECTED_CATEGORY: &str = "article";
    const EXPECTED_KEY: &str = "1";
    const ACM_TEXT: &str = r"
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
";
    const IEEE_TEXT: &str = "
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
";
    const SCI_DIR_TEXT: &str = "
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
author = {M. Krakowski and K. Kurek and Ł. Nogal},
keywords = {Hardware-in-the-loop, Real time simulations, DAQ Cards, IEC 61850, Real time Linux},
}
";
    const SCOPUS_TEXT: &str = "
Scopus
EXPORT DATE: 02 July 2024

@ARTICLE{1,
	author = {Chamana, Manohar and Bhatta, Rabindra and Schmitt, Konrad and Shrestha, Rajendra and Bayne, Stephen},
	title = {An Integrated Testbed for Power System Cyber-Physical Operations Training},
	year = {2023},
	journal = {Applied Sciences (Switzerland)},
	volume = {13},
	number = {16},
	doi = {10.3390/app13169451},
	url = {https://www.scopus.com/inward/record.uri?eid=2-s2.0-85169099191&doi=10.3390%2fapp13169451&partnerID=40&md5=17b896c1c440787efcbc5d384003d31c},
	affiliations = {National Wind Institute, Texas Tech University, Lubbock, 79401, TX, United States; Electrical and Computer Engineering Department, Texas Tech University, Lubbock, 79401, TX, United States},
	author_keywords = {cyberattacks; cyber–physical systems; education; power systems; real-time testbed; smart grids},
	correspondence_address = {R. Bhatta; National Wind Institute, Texas Tech University, Lubbock, 79401, United States; email: rabindra.bhatta(at)ttu.edu},
	publisher = {Multidisciplinary Digital Publishing Institute (MDPI)},
	issn = {20763417},
	language = {English},
	abbrev_source_title = {Appl. Sci.},
	type = {Article},
	publication_stage = {Final},
	source = {Scopus},
	note = {Cited by: 3; All Open Access, Gold Open Access}
}

@ARTICLE{2,
	author = {Tabish, Nimra and Chaur-Luh, Tsai},
	title = {Maritime Autonomous Surface Ships: A Review of Cybersecurity Challenges, Countermeasures, and Future Perspectives},
	year = {2024},
	journal = {IEEE Access},
	volume = {12},
	pages = {17114 – 17136},
	doi = {10.1109/ACCESS.2024.3357082},
	url = {https://www.scopus.com/inward/record.uri?eid=2-s2.0-85184014406&doi=10.1109%2fACCESS.2024.3357082&partnerID=40&md5=45e865ea0976a8c03ec29d3410837818},
	affiliations = {National Kaohsiung University of Science and Technology (NKUST), Department of Maritime Science and Technology, Kaohsiung City, 81157, Taiwan},
	author_keywords = {Cyber security; cyberattack detection; intrusion detection systems; marine autonomous surface ships; prevention and countermeasures},
	keywords = {Computer crime; Cryptography; Cybersecurity; Interactive computer systems; Intrusion detection; Ships; Autonomous Vehicles; Cyber security; Cyber-attacks; Cyberattack detection; Guideline; Intrusion Detection Systems; Intrusion-Detection; Marine autonomous surface ship; Marine vehicles; Prevention and countermeasure; Real - Time system; Surface ship; Real time systems},
	correspondence_address = {T. Chaur-Luh; National Kaohsiung University of Science and Technology (NKUST), Department of Maritime Science and Technology, Kaohsiung City, 81157, Taiwan; email: chaurluh(at)nkust.edu.tw},
	publisher = {Institute of Electrical and Electronics Engineers Inc.},
	issn = {21693536},
	language = {English},
	abbrev_source_title = {IEEE Access},
	type = {Article},
	publication_stage = {Final},
	source = {Scopus},
	note = {Cited by: 0; All Open Access, Gold Open Access}
}
";

    use std::io::Cursor;
    pub struct ExpectedNextEntry {
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
    impl ExpectedNextEntry {
        pub fn new() -> [Self; 4] {
            let acm = ExpectedNextEntry {
                file: Cursor::new(String::from(ACM_TEXT)),
                expected_entry1: Cursor::new(Vec::from(&ACM_TEXT[..623])),
                expected_tell1: 623,
                expected_entry2: Cursor::new(Vec::from(&ACM_TEXT[623..1133])),
                expected_tell2: 1133,
                expected_entry3: Cursor::new(Vec::from(&ACM_TEXT[1133..])),
                expected_tell3: 1134,
                expected_entry4: Cursor::new(Vec::from(&ACM_TEXT[1134..])),
                expected_tell4: 1134,
            };

            let ieee = ExpectedNextEntry {
                file: Cursor::new(String::from(IEEE_TEXT)),
                expected_entry1: Cursor::new(Vec::from(&IEEE_TEXT[..357])),
                expected_tell1: 357,
                expected_entry2: Cursor::new(Vec::from(&IEEE_TEXT[357..738])),
                expected_tell2: 738,
                expected_entry3: Cursor::new(Vec::from(&IEEE_TEXT[738..])),
                expected_tell3: 739,
                expected_entry4: Cursor::new(Vec::from(&IEEE_TEXT[739..])),
                expected_tell4: 739,
            };

            let science_directory = ExpectedNextEntry {
                file: Cursor::new(String::from(SCI_DIR_TEXT)),
                expected_entry1: Cursor::new(Vec::from(&SCI_DIR_TEXT[..542])),
                expected_tell1: 542,
                expected_entry2: Cursor::new(Vec::from(&SCI_DIR_TEXT[542..1113])),
                expected_tell2: 1113,
                expected_entry3: Cursor::new(Vec::from(&SCI_DIR_TEXT[1113..])),
                expected_tell3: 1114,
                expected_entry4: Cursor::new(Vec::from(&SCI_DIR_TEXT[1114..])),
                expected_tell4: 1114,
            };

            let scopus = ExpectedNextEntry {
                file: Cursor::new(String::from(SCOPUS_TEXT)),
                expected_entry1: Cursor::new(Vec::<u8>::from(&SCOPUS_TEXT[..1275])),
                expected_tell1: 1275,
                expected_entry2: Cursor::new(Vec::<u8>::from(&SCOPUS_TEXT[1275..2942])),
                expected_tell2: 2942,
                expected_entry3: Cursor::new(Vec::<u8>::from(&SCOPUS_TEXT[2942..])),
                expected_tell3: 2943,
                expected_entry4: Cursor::new(Vec::<u8>::from(&SCOPUS_TEXT[2943..])),
                expected_tell4: 2943,
            };

            [acm, ieee, science_directory, scopus]
        }
    }

    pub struct ExpectedGetCategory {
        pub entry: Cursor<Vec<u8>>,
        pub tell: u64,
        pub category: String,
    }
    impl ExpectedGetCategory {
        pub fn new() -> [Self; 4] {
            let acm = ExpectedGetCategory {
                entry: Cursor::new(Vec::from(&ACM_TEXT[..623])),
                tell: 10,
                category: EXPECTED_CATEGORY.to_string(),
            };
            let ieee = ExpectedGetCategory {
                entry: Cursor::new(Vec::from(&IEEE_TEXT[..357])),
                tell: 10,
                category: EXPECTED_CATEGORY.to_string(),
            };
            let science_directory = ExpectedGetCategory {
                entry: Cursor::new(Vec::from(&SCI_DIR_TEXT[..542])),
                tell: 10,
                category: EXPECTED_CATEGORY.to_string(),
            };
            let scopus = ExpectedGetCategory {
                entry: Cursor::new(Vec::<u8>::from(&SCOPUS_TEXT[..1275])),
                tell: 44,
                category: EXPECTED_CATEGORY.to_string(),
            };
            [acm, ieee, science_directory, scopus]
        }
    }

    pub struct ExpectedValue {
        pub value: String,
        pub tell: u64,
    }
    pub struct CaseGetKey {
        pub entry: Cursor<Vec<u8>>,
        pub expected: ExpectedValue,
    }
    impl CaseGetKey {
        pub fn new() -> [Self; 4] {
            let acm = CaseGetKey {
                entry: Cursor::new(Vec::from(&ACM_TEXT[..623])),
                expected: ExpectedValue {
                    value: EXPECTED_KEY.to_string(),
                    tell: 12,
                },
            };
            let ieee = CaseGetKey {
                entry: Cursor::new(Vec::from(&IEEE_TEXT[..357])),
                expected: ExpectedValue {
                    value: EXPECTED_KEY.to_string(),
                    tell: 12,
                },
            };
            let science_directory = CaseGetKey {
                entry: Cursor::new(Vec::from(&SCI_DIR_TEXT[..542])),
                expected: ExpectedValue {
                    value: EXPECTED_KEY.to_string(),
                    tell: 12,
                },
            };
            let scopus = CaseGetKey {
                entry: Cursor::new(Vec::<u8>::from(&SCOPUS_TEXT[..1275])),
                expected: ExpectedValue {
                    value: EXPECTED_KEY.to_string(),
                    tell: 46,
                },
            };
            [acm, ieee, science_directory, scopus]
        }
    }

    pub struct CaseGetElementKey {
        pub entry: Cursor<Vec<u8>>,
        pub expected: ExpectedValue,
    }
    impl CaseGetElementKey {
        pub fn new() -> [Self; 4] {
            let acm = CaseGetElementKey {
                entry: Cursor::new(Vec::from(&ACM_TEXT[..623])),
                expected: ExpectedValue {
                    value: String::from("author"),
                    tell: 21,
                },
            };
            let ieee = CaseGetElementKey {
                entry: Cursor::new(Vec::from(&IEEE_TEXT[..357])),
                expected: ExpectedValue {
                    value: String::from("author"),
                    tell: 22,
                },
            };
            let science_directory = CaseGetElementKey {
                entry: Cursor::new(Vec::from(&SCI_DIR_TEXT[..542])),
                expected: ExpectedValue {
                    value: String::from("title"),
                    tell: 20,
                },
            };
            let scopus = CaseGetElementKey {
                entry: Cursor::new(Vec::<u8>::from(&SCOPUS_TEXT[..1275])),
                expected: ExpectedValue {
                    value: String::from("author"),
                    tell: 56,
                },
            };
            [acm, ieee, science_directory, scopus]
        }
    }

    pub struct CaseGetElementValue {
        pub entry: Cursor<Vec<u8>>,
        pub expected: ExpectedValue,
    }
    impl CaseGetElementValue {
        pub fn new() -> [Self; 4] {
            let acm = CaseGetElementValue {
                entry: Cursor::new(Vec::from(&ACM_TEXT[..623])),
                expected: ExpectedValue {
                    value: String::from(r"Ahmad, Waqar and Hasan, Osman and Tahar, Sofi\`{e}ne"),
                    tell: 76,
                },
            };
            let ieee = CaseGetElementValue {
                entry: Cursor::new(Vec::from(&IEEE_TEXT[..357])),
                expected: ExpectedValue {
                    value: String::from(
                        "Wang, Wenlong and Liu, Minghui and Zhao, Xicai and Yang, Gui",
                    ),
                    tell: 84,
                },
            };
            let science_directory = CaseGetElementValue {
                entry: Cursor::new(Vec::from(&SCI_DIR_TEXT[..542])),
                expected: ExpectedValue {
                    value: String::from("Research and implementation of virtual circuit test tool for smart substations"),
                    tell: 101,
                },
            };
            let scopus = CaseGetElementValue {
                entry: Cursor::new(Vec::<u8>::from(&SCOPUS_TEXT[..1275])),
                expected: ExpectedValue {
                    value: String::from("Chamana, Manohar and Bhatta, Rabindra and Schmitt, Konrad and Shrestha, Rajendra and Bayne, Stephen"),
                    tell: 158,
                },
            };
            [acm, ieee, science_directory, scopus]
        }
    }

    pub struct ExpectedElement {
        pub key: String,
        pub value: String,
        pub tell: u64,
    }
    pub struct CaseGetNextElement {
        pub entry: Cursor<Vec<u8>>,
        pub expected: ExpectedElement,
    }
    impl CaseGetNextElement {
        pub fn new() -> [Self; 4] {
            let acm = CaseGetNextElement {
                entry: Cursor::new(Vec::from(&ACM_TEXT[..623])),
                expected: ExpectedElement {
                    key: String::from("author"),
                    value: String::from(r"Ahmad, Waqar and Hasan, Osman and Tahar, Sofi\`{e}ne"),
                    tell: 76,
                },
            };
            let ieee = CaseGetNextElement {
                entry: Cursor::new(Vec::from(&IEEE_TEXT[..357])),
                expected: ExpectedElement {
                    key: String::from("author"),
                    value: String::from(
                        "Wang, Wenlong and Liu, Minghui and Zhao, Xicai and Yang, Gui",
                    ),
                    tell: 84,
                },
            };
            let science_directory = CaseGetNextElement {
                entry: Cursor::new(Vec::from(&SCI_DIR_TEXT[..542])),
                expected: ExpectedElement {
                    key: String::from("title"),
                    value: String::from("Research and implementation of virtual circuit test tool for smart substations"),
                    tell: 101,
                },
            };
            let scopus = CaseGetNextElement {
                entry: Cursor::new(Vec::<u8>::from(&SCOPUS_TEXT[..1275])),
                expected: ExpectedElement {
                    key: String::from("author"),
                    value: String::from("Chamana, Manohar and Bhatta, Rabindra and Schmitt, Konrad and Shrestha, Rajendra and Bayne, Stephen"),
                    tell: 158,
                },
            };
            [acm, ieee, science_directory, scopus]
        }
    }
}
