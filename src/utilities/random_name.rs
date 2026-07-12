use rand::RngExt;

const GIVEN_NAMES: &[&str] = &[
    "子涵",
    "梓轩",
    "雨桐",
    "欣怡",
    "浩然",
    "铁蛋",
    "佳怡",
    "宇轩",
    "梦瑶",
    "俊杰",
    "思涵",
    "晨曦",
    "嘉豪",
    "诗雨",
    "若曦",
    "浩宇",
];

const XINGSHI: &[&str] = &[
    "王", "李", "张", "刘", "陈",
    "杨", "赵", "黄", "周", "吴",
];

pub fn random_chinese_name() -> String
{
    let mut rng = rand::rng();

    format!(
        "{}{}",
        XINGSHI[rng.random_range(0..XINGSHI.len())],
        GIVEN_NAMES[rng.random_range(0..GIVEN_NAMES.len())]
    )
}