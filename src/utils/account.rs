use std::collections::HashMap;

use crate::session::SignSession;

use super::sql::DataBase;

// 添加账号。TODO: 跳过输入密码阶段
pub async fn add_account(db: &DataBase, uname: String, pwd: Option<String>) {
    let pwd = if let Some(pwd) = pwd {
        pwd
    } else {
        inquire::Password::new("密码：")
            .without_confirmation()
            .prompt()
            .unwrap()
    };
    let enc_pwd = crate::utils::encrypto_pwd(&pwd);
    let session = SignSession::login(&uname, &enc_pwd).await.unwrap();
    let name = session.get_stu_name();
    db.add_account_or(&uname, &enc_pwd, name, DataBase::update_account);
    let courses = session.get_courses().await.unwrap();
    for c in courses {
        db.add_course_or(&c, |_, _| {});
    }
}
// 添加账号（刷新时用，此时密码一定是存在的且为加密后的密码）。
pub async fn add_account_enc(db: &DataBase, uname: String, enc_pwd: &str) {
    let session = SignSession::login(&uname, &enc_pwd).await.unwrap();
    let name = session.get_stu_name();
    db.add_account_or(&uname, &enc_pwd, name, DataBase::update_account);
    let courses = session.get_courses().await.unwrap();
    for c in courses {
        db.add_course_or(&c, |_, _| {});
    }
}

pub async fn get_sessions(db: &DataBase) -> HashMap<String, SignSession> {
    let accounts = db.get_accounts();
    let config_dir = crate::utils::CONFIG_DIR.clone();
    let mut s = HashMap::new();
    for a in accounts {
        let cookies_dir = config_dir.join(a.0.to_string() + ".json");
        let session = SignSession::load(cookies_dir).await.unwrap();
        s.insert(a.0, session);
    }
    s
}