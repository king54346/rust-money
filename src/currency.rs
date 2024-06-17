use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::formatter::Formatter;

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct Currency {
    //货币的代码
    pub(crate) code: String,
    //货币的数字代码
    numeric_code: &'static str,
    //分数单位
    pub fraction: usize,
    //货币的符号
    grapheme: &'static str,
    //显示货币的模板
    template: &'static str,
    //表示小数点符号
    decimal: &'static str,
    //千位分隔符
    thousand: &'static str,
}

impl PartialEq for Currency {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl Currency {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_uppercase(),
            // 其他字段的初始化可以在这里添加
            numeric_code: "",
            fraction: 0,
            grapheme: "",
            template: "",
            decimal: "",
            thousand: "",
        }
    }
    pub fn new_all(code: &str, grapheme:&'static str,numeric_code:&'static str,template:&'static str, decimal:&'static str, thousand:&'static str, fraction:usize) -> Self {
        Self {
            code: code.to_uppercase(),
            // 其他字段的初始化可以在这里添加
            numeric_code,
            fraction,
            grapheme,
            template,
            decimal,
            thousand,
        }
    }
    pub fn get(&mut self) {
        match Currencies::default().get_currency_by_code(self.code.as_str()) {
            None => {
            }
            Some(v) => {
                self.decimal=v.decimal;
                self.fraction=v.fraction;
                self.numeric_code=v.numeric_code;
                self.grapheme=v.grapheme;
                self.template=v.template;
                self.thousand=v.thousand;
            }
        }
    }
    pub fn formatter(&self) -> Formatter {
        Formatter::new(self.fraction,self.decimal,self.thousand,self.grapheme,self.template)
    }
}

impl Eq for Currency {}

pub struct Currencies {
    currencies: HashMap<String, Currency>,
}

impl Currencies {
    pub fn new() -> Self {
        Self {
            currencies: HashMap::new(),
        }
    }

    pub fn currency_by_numeric_code(&self, code: &str) -> Option<&Currency> {
        self.currencies.values().find(|c| c.numeric_code == code)
    }

    pub fn get_currency_by_code(&self, code: &str) -> Option<&Currency> {
        self.currencies.get(code.to_uppercase().as_str())
    }

    pub fn add(&mut self, currency: Currency) {
        self.currencies.insert(currency.code.clone(), currency);
    }

    fn add_bulk(&mut self, currencies: Vec<Currency>) {
        for currency in currencies {
            self.add(currency);
        }
    }

}

impl Default for Currencies {
    fn default() -> Self {
        let mut currencies = Currencies::new();
        let currency_list = vec![
            Currency { code: "AED".to_string(), numeric_code: "784", fraction: 2, grapheme: ".د.إ", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "AFN".to_string(), numeric_code: "971", fraction: 2, grapheme: "؋", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "ALL".to_string(), numeric_code: "008", fraction: 2, grapheme: "L", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "AMD".to_string(), numeric_code: "051", fraction: 2, grapheme: "դր.", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "ANG".to_string(), numeric_code: "532", fraction: 2, grapheme: "ƒ", template: "$1", decimal: ",", thousand: "." },
            Currency { code: "AOA".to_string(), numeric_code: "973", fraction: 2, grapheme: "Kz", template: "1$", decimal: ".", thousand: "," },
            Currency { code: "ARS".to_string(), numeric_code: "032", fraction: 2, grapheme: "$", template: "$1", decimal: ",", thousand: "." },
            Currency { code: "AUD".to_string(), numeric_code: "036", fraction: 2, grapheme: "$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "AWG".to_string(), numeric_code: "533", fraction: 2, grapheme: "ƒ", template: "1$", decimal: ".", thousand: "," },
            Currency { code: "AZN".to_string(), numeric_code: "944", fraction: 2, grapheme: "₼", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "BAM".to_string(), numeric_code: "977", fraction: 2, grapheme: "KM", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "BBD".to_string(), numeric_code: "052", fraction: 2, grapheme: "$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "BDT".to_string(), numeric_code: "050", fraction: 2, grapheme: "৳", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "BGN".to_string(), numeric_code: "975", fraction: 2, grapheme: "лв", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "BHD".to_string(), numeric_code: "048", fraction: 3, grapheme: ".د.ب", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "BIF".to_string(), numeric_code: "108", fraction: 0, grapheme: "Fr", template: "1$", decimal: ".", thousand: "," },
            Currency { code: "BMD".to_string(), numeric_code: "060", fraction: 2, grapheme: "$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "BND".to_string(), numeric_code: "096", fraction: 2, grapheme: "$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "BOB".to_string(), numeric_code: "068", fraction: 2, grapheme: "Bs.", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "BRL".to_string(), numeric_code: "986", fraction: 2, grapheme: "R$", template: "$1", decimal: ",", thousand: "." },
            Currency { code: "BSD".to_string(), numeric_code: "044", fraction: 2, grapheme: "$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "BTN".to_string(), numeric_code: "064", fraction: 2, grapheme: "Nu.", template: "1$", decimal: ".", thousand: "," },
            Currency { code: "BWP".to_string(), numeric_code: "072", fraction: 2, grapheme: "P", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "BYN".to_string(), numeric_code: "933", fraction: 2, grapheme: "р.", template: "1 $", decimal: ",", thousand: " " },
            Currency { code: "BYR".to_string(), numeric_code: "", fraction: 0, grapheme: "р.", template: "1 $", decimal: ",", thousand: " " },
            Currency { code: "BZD".to_string(), numeric_code: "084", fraction: 2, grapheme: "BZ$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "CAD".to_string(), numeric_code: "124", fraction: 2, grapheme: "$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "CDF".to_string(), numeric_code: "976", fraction: 2, grapheme: "FC", template: "1$", decimal: ".", thousand: "," },
            Currency { code: "CHF".to_string(), numeric_code: "756", fraction: 2, grapheme: "CHF", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "CLF".to_string(), numeric_code: "990", fraction: 4, grapheme: "UF", template: "$1", decimal: ",", thousand: "." },
            Currency { code: "CLP".to_string(), numeric_code: "152", fraction: 0, grapheme: "$", template: "$1", decimal: ",", thousand: "." },
            Currency { code: "CNY".to_string(), numeric_code: "156", fraction: 2, grapheme: "元", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "COP".to_string(), numeric_code: "170", fraction: 2, grapheme: "$", template: "$1", decimal: ",", thousand: "." },
            Currency { code: "CRC".to_string(), numeric_code: "188", fraction: 2, grapheme: "₡", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "CUC".to_string(), numeric_code: "931", fraction: 2, grapheme: "$", template: "1$", decimal: ".", thousand: "," },
            Currency { code: "CUP".to_string(), numeric_code: "192", fraction: 2, grapheme: "$MN", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "CVE".to_string(), numeric_code: "132", fraction: 2, grapheme: "$", template: "1$", decimal: ".", thousand: "," },
            Currency { code: "CZK".to_string(), numeric_code: "203", fraction: 2, grapheme: "Kč", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "DJF".to_string(), numeric_code: "262", fraction: 0, grapheme: "Fdj", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "DKK".to_string(), numeric_code: "208", fraction: 2, grapheme: "kr", template: "$ 1", decimal: ",", thousand: "." },
            Currency { code: "DOP".to_string(), numeric_code: "214", fraction: 2, grapheme: "RD$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "DZD".to_string(), numeric_code: "012", fraction: 2, grapheme: ".د.ج", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "EEK".to_string(), numeric_code: "", fraction: 2, grapheme: "kr", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "EGP".to_string(), numeric_code: "818", fraction: 2, grapheme: "£", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "ERN".to_string(), numeric_code: "232", fraction: 2, grapheme: "Nfk", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "ETB".to_string(), numeric_code: "230", fraction: 2, grapheme: "Br", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "EUR".to_string(), numeric_code: "978", fraction: 2, grapheme: "€", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "FJD".to_string(), numeric_code: "242", fraction: 2, grapheme: "$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "FKP".to_string(), numeric_code: "238", fraction: 2, grapheme: "£", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "GBP".to_string(), numeric_code: "826", fraction: 2, grapheme: "£", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "GEL".to_string(), numeric_code: "981", fraction: 2, grapheme: "ლ", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "GGP".to_string(), numeric_code: "", fraction: 2, grapheme: "£", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "GHC".to_string(), numeric_code: "", fraction: 2, grapheme: "¢", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "GHS".to_string(), numeric_code: "936", fraction: 2, grapheme: "₵", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "GIP".to_string(), numeric_code: "292", fraction: 2, grapheme: "£", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "GMD".to_string(), numeric_code: "270", fraction: 2, grapheme: "D", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "GNF".to_string(), numeric_code: "324", fraction: 0, grapheme: "FG", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "GTQ".to_string(), numeric_code: "320", fraction: 2, grapheme: "Q", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "GYD".to_string(), numeric_code: "328", fraction: 2, grapheme: "$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "HKD".to_string(), numeric_code: "344", fraction: 2, grapheme: "$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "HNL".to_string(), numeric_code: "340", fraction: 2, grapheme: "L", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "HRK".to_string(), numeric_code: "191", fraction: 2, grapheme: "kn", template: "1 $", decimal: ",", thousand: "." },
            Currency { code: "HTG".to_string(), numeric_code: "332", fraction: 2, grapheme: "G", template: "1 $", decimal: ",", thousand: "." },
            Currency { code: "HUF".to_string(), numeric_code: "348", fraction: 2, grapheme: "Ft", template: "1 $", decimal: ",", thousand: "." },
            Currency { code: "IDR".to_string(), numeric_code: "360", fraction: 2, grapheme: "Rp", template: "$1", decimal: ",", thousand: "." },
            Currency { code: "ILS".to_string(), numeric_code: "376", fraction: 2, grapheme: "₪", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "IMP".to_string(), numeric_code: "", fraction: 2, grapheme: "£", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "INR".to_string(), numeric_code: "356", fraction: 2, grapheme: "₹", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "IQD".to_string(), numeric_code: "368", fraction: 3, grapheme: ".د.ع", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "IRR".to_string(), numeric_code: "364", fraction: 2, grapheme: "﷼", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "ISK".to_string(), numeric_code: "352", fraction: 0, grapheme: "kr", template: "$1", decimal: ",", thousand: "." },
            Currency { code: "JEP".to_string(), numeric_code: "", fraction: 2, grapheme: "£", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "JMD".to_string(), numeric_code: "388", fraction: 2, grapheme: "J$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "JOD".to_string(), numeric_code: "400", fraction: 3, grapheme: ".د.إ", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "JPY".to_string(), numeric_code: "392", fraction: 0, grapheme: "¥", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "KES".to_string(), numeric_code: "404", fraction: 2, grapheme: "KSh", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "KGS".to_string(), numeric_code: "417", fraction: 2, grapheme: "сом", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "KHR".to_string(), numeric_code: "116", fraction: 2, grapheme: "៛", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "KMF".to_string(), numeric_code: "174", fraction: 0, grapheme: "CF", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "KPW".to_string(), numeric_code: "408", fraction: 2, grapheme: "₩", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "KRW".to_string(), numeric_code: "410", fraction: 0, grapheme: "₩", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "KWD".to_string(), numeric_code: "414", fraction: 3, grapheme: ".د.ك", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "KYD".to_string(), numeric_code: "136", fraction: 2, grapheme: "$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "KZT".to_string(), numeric_code: "398", fraction: 2, grapheme: "₸", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "LAK".to_string(), numeric_code: "418", fraction: 2, grapheme: "₭", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "LBP".to_string(), numeric_code: "422", fraction: 2, grapheme: "£", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "LKR".to_string(), numeric_code: "144", fraction: 2, grapheme: "₨", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "LRD".to_string(), numeric_code: "430", fraction: 2, grapheme: "$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "LSL".to_string(), numeric_code: "426", fraction: 2, grapheme: "L", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "LTL".to_string(), numeric_code: "", fraction: 2, grapheme: "Lt", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "LVL".to_string(), numeric_code: "", fraction: 2, grapheme: "Ls", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "LYD".to_string(), numeric_code: "434", fraction: 3, grapheme: ".د.ل", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "MAD".to_string(), numeric_code: "504", fraction: 2, grapheme: ".د.م", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "MDL".to_string(), numeric_code: "498", fraction: 2, grapheme: "lei", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "MGA".to_string(), numeric_code: "969", fraction: 2, grapheme: "Ar", template: "1$", decimal: ".", thousand: "," },
            Currency { code: "MKD".to_string(), numeric_code: "807", fraction: 2, grapheme: "ден", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "MMK".to_string(), numeric_code: "104", fraction: 2, grapheme: "K", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "MNT".to_string(), numeric_code: "496", fraction: 2, grapheme: "₮", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "MOP".to_string(), numeric_code: "446", fraction: 2, grapheme: "P", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "MRU".to_string(), numeric_code: "929", fraction: 2, grapheme: "UM", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "MUR".to_string(), numeric_code: "480", fraction: 2, grapheme: "₨", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "MVR".to_string(), numeric_code: "462", fraction: 2, grapheme: "MVR", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "MWK".to_string(), numeric_code: "454", fraction: 2, grapheme: "MK", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "MXN".to_string(), numeric_code: "484", fraction: 2, grapheme: "$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "MYR".to_string(), numeric_code: "458", fraction: 2, grapheme: "RM", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "MZN".to_string(), numeric_code: "943", fraction: 2, grapheme: "MT", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "NAD".to_string(), numeric_code: "516", fraction: 2, grapheme: "$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "NGN".to_string(), numeric_code: "566", fraction: 2, grapheme: "₦", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "NIO".to_string(), numeric_code: "558", fraction: 2, grapheme: "C$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "NOK".to_string(), numeric_code: "578", fraction: 2, grapheme: "kr", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "NPR".to_string(), numeric_code: "524", fraction: 2, grapheme: "₨", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "NZD".to_string(), numeric_code: "554", fraction: 2, grapheme: "$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "OMR".to_string(), numeric_code: "512", fraction: 3, grapheme: "﷼", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "PAB".to_string(), numeric_code: "590", fraction: 2, grapheme: "B/.", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "PEN".to_string(), numeric_code: "604", fraction: 2, grapheme: "S/", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "PGK".to_string(), numeric_code: "598", fraction: 2, grapheme: "K", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "PHP".to_string(), numeric_code: "608", fraction: 2, grapheme: "₱", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "PKR".to_string(), numeric_code: "586", fraction: 2, grapheme: "₨", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "PLN".to_string(), numeric_code: "985", fraction: 2, grapheme: "zł", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "PYG".to_string(), numeric_code: "600", fraction: 0, grapheme: "Gs", template: "1$", decimal: ".", thousand: "," },
            Currency { code: "QAR".to_string(), numeric_code: "634", fraction: 2, grapheme: "﷼", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "RON".to_string(), numeric_code: "946", fraction: 2, grapheme: "lei", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "RSD".to_string(), numeric_code: "941", fraction: 2, grapheme: "дин.", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "RUB".to_string(), numeric_code: "643", fraction: 2, grapheme: "₽", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "RUR".to_string(), numeric_code: "", fraction: 2, grapheme: "₽", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "RWF".to_string(), numeric_code: "646", fraction: 0, grapheme: "FRw", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "SAR".to_string(), numeric_code: "682", fraction: 2, grapheme: "﷼", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "SBD".to_string(), numeric_code: "090", fraction: 2, grapheme: "$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "SCR".to_string(), numeric_code: "690", fraction: 2, grapheme: "₨", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "SDG".to_string(), numeric_code: "938", fraction: 2, grapheme: "£", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "SEK".to_string(), numeric_code: "752", fraction: 2, grapheme: "kr", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "SGD".to_string(), numeric_code: "702", fraction: 2, grapheme: "$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "SHP".to_string(), numeric_code: "654", fraction: 2, grapheme: "£", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "SKK".to_string(), numeric_code: "", fraction: 2, grapheme: "Sk", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "SLE".to_string(), numeric_code: "925", fraction: 2, grapheme: "Le", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "SLL".to_string(), numeric_code: "694", fraction: 2, grapheme: "Le", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "SOS".to_string(), numeric_code: "706", fraction: 2, grapheme: "Sh", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "SRD".to_string(), numeric_code: "968", fraction: 2, grapheme: "$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "SSP".to_string(), numeric_code: "728", fraction: 2, grapheme: "£", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "STD".to_string(), numeric_code: "", fraction: 2, grapheme: "Db", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "STN".to_string(), numeric_code: "930", fraction: 2, grapheme: "Db", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "SVC".to_string(), numeric_code: "222", fraction: 2, grapheme: "₡", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "SYP".to_string(), numeric_code: "760", fraction: 2, grapheme: "£", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "SZL".to_string(), numeric_code: "748", fraction: 2, grapheme: "£", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "THB".to_string(), numeric_code: "764", fraction: 2, grapheme: "฿", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "TJS".to_string(), numeric_code: "972", fraction: 2, grapheme: "SM", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "TMT".to_string(), numeric_code: "934", fraction: 2, grapheme: "T", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "TND".to_string(), numeric_code: "788", fraction: 3, grapheme: ".د.ت", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "TOP".to_string(), numeric_code: "776", fraction: 2, grapheme: "T$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "TRL".to_string(), numeric_code: "", fraction: 2, grapheme: "₤", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "TRY".to_string(), numeric_code: "949", fraction: 2, grapheme: "₺", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "TTD".to_string(), numeric_code: "780", fraction: 2, grapheme: "TT$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "TWD".to_string(), numeric_code: "901", fraction: 2, grapheme: "NT$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "TZS".to_string(), numeric_code: "834", fraction: 0, grapheme: "TSh", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "UAH".to_string(), numeric_code: "980", fraction: 2, grapheme: "₴", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "UGX".to_string(), numeric_code: "800", fraction: 0, grapheme: "USh", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "USD".to_string(), numeric_code: "840", fraction: 2, grapheme: "$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "UYU".to_string(), numeric_code: "858", fraction: 2, grapheme: "$U", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "UZS".to_string(), numeric_code: "860", fraction: 2, grapheme: "so‘m", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "VEF".to_string(), numeric_code: "937", fraction: 2, grapheme: "Bs", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "VES".to_string(), numeric_code: "928", fraction: 2, grapheme: "Bs.S", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "VND".to_string(), numeric_code: "704", fraction: 0, grapheme: "₫", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "VUV".to_string(), numeric_code: "548", fraction: 0, grapheme: "Vt", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "WST".to_string(), numeric_code: "882", fraction: 2, grapheme: "T", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "XAF".to_string(), numeric_code: "950", fraction: 0, grapheme: "Fr", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "XAG".to_string(), numeric_code: "961", fraction: 0, grapheme: "oz t", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "XAU".to_string(), numeric_code: "959", fraction: 0, grapheme: "oz t", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "XCD".to_string(), numeric_code: "951", fraction: 2, grapheme: "$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "XDR".to_string(), numeric_code: "960", fraction: 0, grapheme: "SDR", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "XOF".to_string(), numeric_code: "952", fraction: 0, grapheme: "CFA", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "XPF".to_string(), numeric_code: "953", fraction: 0, grapheme: "₣", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "YER".to_string(), numeric_code: "886", fraction: 2, grapheme: "﷼", template: "1 $", decimal: ".", thousand: "," },
            Currency { code: "ZAR".to_string(), numeric_code: "710", fraction: 2, grapheme: "R", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "ZMW".to_string(), numeric_code: "967", fraction: 2, grapheme: "ZK", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "ZWD".to_string(), numeric_code: "716", fraction: 2, grapheme: "Z$", template: "$1", decimal: ".", thousand: "," },
            Currency { code: "ZWL".to_string(), numeric_code: "932", fraction: 2, grapheme: "Z$", template: "$1", decimal: ".", thousand: "," },
        ];
        currencies.add_bulk(currency_list);
        currencies
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_get(){
        let currencies = Currencies::default();
        let option = currencies.get_currency_by_code("EUR");
        println!("{:?}", option);
    }
    #[test]
    fn test_currency_add(){
        let mut currencies = Currencies::default();
        let currency = Currency::new_all("ssdfd","sss","","","","",32);
        currencies.add(currency);
        let option = currencies.get_currency_by_code("ssdfd");
        println!("{:?}", option);
    }
    #[test]
    fn test_currency_equals(){
        let currencies = Currencies::default();
        let option = currencies.get_currency_by_code("EUR").unwrap();
        let option2 = currencies.get_currency_by_code("USD").unwrap();
        let option3 = currencies.get_currency_by_code("EUR").unwrap();
        println!("{:?}", option.eq(option2));
        println!("{:?}", option.eq(option3));
    }
}