use crate::_structs::_xl::shared_strings;
use crate::_traits::_xl::shared_strings::{Extract, Si};
use regex::Regex;

const TAG_REGEX: &str = "^#<.+>$";

impl Extract for shared_strings::sst {
    fn extract(&self) -> anyhow::Result<Vec<String>> {
        let re = Regex::new(TAG_REGEX)?;
        match self.si {
            None => Ok(Vec::new()),
            Some(ref si) => Ok(si
                .iter()
                .filter(|si| -> bool {
                    match si.value() {
                        None => false,
                        Some(t) => re.is_match(&t),
                    }
                })
                .map(|si| -> String { si.value().unwrap() })
                .collect()),
        }
    }
}

impl Si for shared_strings::si {
    fn value(&self) -> Option<String> {
        match &self.r {
            None => match &self.t {
                None => None,
                Some(t) => Some(t.t.to_string()),
            },
            Some(r) => {
                let str: String = r.iter().fold(
                    String::new(),
                    |a: String, b: &shared_strings::R| -> String { a + &b.t.t },
                );
                Some(str)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        _structs::_xl::shared_strings::{si, sst, R, T},
        _traits::_xl::shared_strings::{Extract, Si},
    };
    use quick_xml::de::from_str;

    const XML: &str = r#"
    <sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" count="19" uniqueCount="11">
        <si>
            <t>pref_name</t>
        </si>
        <si>
            <t>sex</t>
        </si>
        <si>
            <t>population</t>
        </si>
        <si>
            <t>徳島</t>
        </si>
        <si>
            <t>香川</t>
        </si>
        <si>
            <t>愛媛</t>
        </si>
        <si>
            <t>高知</t>
        </si>
        <si>
            <t>福岡</t>
        </si>
        <si>
            <t>佐賀</t>
        </si>
        <si>
            <t>長崎</t>
        </si>
        <si>
            <t>東京</t>
        </si>
        <si>
            <t>#&lt;基本情報&gt;&lt;ほげ&gt;</t>
        </si>
        <si>
            <r>
                <rPr>
                    <sz val="10"/>
                    <color rgb="FF000000"/>
                    <rFont val="Arial"/>
                    <family val="0"/>
                    <charset val="1"/>
                </rPr>
                <t xml:space="preserve">#&lt;</t>
            </r>
            <r>
                <rPr>
                    <sz val="10"/>
                    <color rgb="FF000000"/>
                    <rFont val="Noto Sans CJK JP"/>
                    <family val="2"/>
                </rPr>
                <t xml:space="preserve">基本情報</t>
            </r>
            <r>
                <rPr>
                    <sz val="10"/>
                    <color rgb="FF000000"/>
                    <rFont val="Arial"/>
                    <family val="0"/>
                    <charset val="1"/>
                </rPr>
                <t xml:space="preserve">&gt;&lt;</t>
            </r>
            <r>
                <rPr>
                    <sz val="10"/>
                    <color rgb="FF000000"/>
                    <rFont val="Noto Sans CJK JP"/>
                    <family val="2"/>
                </rPr>
                <t xml:space="preserve">氏名</t>
            </r>
            <r>
                <rPr>
                    <sz val="10"/>
                    <color rgb="FF000000"/>
                    <rFont val="Arial"/>
                    <family val="0"/>
                    <charset val="1"/>
                </rPr>
                <t xml:space="preserve">&gt;</t>
            </r>
        </si>
    </sst>
"#;

    #[test]
    fn extract() {
        let sst: sst = from_str::<sst>(XML).unwrap();
        assert_eq!(
            vec!["#<基本情報><ほげ>", "#<基本情報><氏名>"],
            sst.extract().unwrap()
        )
    }

    #[test]
    fn si_t_value() {
        let si = si {
            t: Some(T {
                t: "DUMMY".to_string(),
            }),
            r: None,
        };
        assert_eq!("DUMMY", si.value().unwrap());
    }
    #[test]
    fn si_r_value() {
        let r: R = R {
            t: T {
                t: "DUMMY".to_string(),
            },
        };
        let si = si {
            t: None,
            r: Some(vec![r]),
        };
        assert_eq!("DUMMY", si.value().unwrap());
    }
}
