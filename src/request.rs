use std::collections::HashMap;

use reqwest::header::HeaderMap;
use serde_json::Value;


pub fn login(id: &String, password: &String) -> Result<String, String> {
    let url: &str = "https://www.dataq.or.kr/www/accounts/login/proc.do";

    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());

    let mut form: HashMap<&str, &str> = HashMap::new();
    form.insert("userperm", "S01");
    form.insert("loginid", id);
    form.insert("loginpw", password);

    let post_client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let res = post_client.post(url)
        .headers(headers)
        .form(&form)
        .send();

    let res = match res {
        Ok(res) => res,
        Err(_) => return Err(String::from("로그인 실패")),
    };
        
    let headers = res.headers().get_all("Set-Cookie");

    for cookies in headers {
        let cookie = cookies.to_str();
        let cookie = match cookie {
            Ok(cookie) => cookie,
            Err(_) => return Err(String::from("로그인 실패")),
        };
        if cookie.contains("JSESSIONID") {
            let session = cookie.split(";").collect::<Vec<&str>>()[0].split("=").collect::<Vec<&str>>()[1];
            return Ok(session.to_string());
        }
    }

    return Ok(String::new()); // Add this line to return an empty string
}

pub fn get_tests(session_id: &String) -> Result<Vec<u64>, String> {
    let url: &str = "https://www.dataq.or.kr/www/mypage/accept/result.dox";

    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("Content-Type", "application/json; charset=UTF-8".parse().unwrap());
    headers.insert("Cookie", format!("JSESSIONID={}", session_id).parse().unwrap());

    let get_client = reqwest::blocking::Client::builder()
        .build()
        .unwrap();

    let res = get_client.post(url)
        .headers(headers)
        .body("{}")
        .send();
    
    let res = match res {
        Ok(res) => res,
        Err(_) => return Err(String::from("시험 결과를 가져오는데 실패했습니다.\n아이디 혹은 비밀번호를 확인해주세요")),
    };

    let text = res.text();
    let text = match text {
        Ok(text) => text,
        Err(_) => return Err(String::from("시험 결과를 가져오는데 실패했습니다.\n아이디 혹은 비밀번호를 확인해주세요")),
    };

    let body: Result<Value, serde_json::Error> = serde_json::from_str(&text);
    let body = match body {
        Ok(body) => body,
        Err(_) => return Err(String::from("시험 결과를 가져오는데 실패했습니다.\n아이디 혹은 비밀번호를 확인해주세요")),
    };

    let list = body["list"].as_array();
    let list = match list {
        Some(list) => list,
        None => return Err(String::from("시험 결과를 가져오는데 실패했습니다.\n아이디 혹은 비밀번호를 확인해주세요")),
    };

    let mut tests: Vec<u64> = Vec::new();

    for test in list {
        let single_test = test["aplySeq"].as_u64();
        let single_test = match single_test {
            Some(single_test) => single_test,
            None => continue,
        };
        tests.push(single_test)
    }

    return Ok(tests)
}

pub fn get_test_results(session_id: &String, test_id: &Vec<u64>, label_text: &mut String) {

    label_text.push_str("채점 되지 않았을 시 0점으로 표기됩니다.\n\n");
    label_text.push_str(format!("총 {}개의 시험 결과를 가져왔습니다.\n\n", test_id.len()).as_str());

    for test in test_id {
        let url: &str = "https://www.dataq.or.kr/www/mypage/accept/score.dox";

        label_text.clear();

        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert("Content-Type", "application/json; charset=UTF-8".parse().unwrap());
        headers.insert("Cookie", format!("JSESSIONID={}", session_id).parse().unwrap());

        let body = format!("{{\"aplySeq\":{}}}", test);

        let post_client = reqwest::blocking::Client::builder()
            .build()
            .unwrap();

        let res = post_client.post(url)
            .headers(headers)
            .body(body)
            .send()
            .unwrap();

        let body: Value = serde_json::from_str(&res.text().unwrap()).unwrap();

        let test: Value = body["info"].clone();

        let test_date = test["examdatetimeSt"].as_str().unwrap();
        let test_name = test["examnm"].as_str().unwrap();
        let test_score = test["hitpoint"].as_f64().unwrap();

        let first_name: &str = test["lecturenm1"].as_str().unwrap();
        let second_name: &str = test["lecturenm2"].as_str().unwrap();

        let first_score: f64 = test["lectureHitpoint1"].as_f64().unwrap();
        let second_score: f64 = test["lectureHitpoint2"].as_f64().unwrap();

        label_text.push_str(format!("시험 날짜: {}\n", test_date).as_str());
        label_text.push_str(format!("시험 이름: {}\n", test_name).as_str());
        label_text.push_str(format!("총 점수: {}\n", test_score).as_str());
        label_text.push_str(format!("과목1: {}, 점수: {}\n", first_name, first_score).as_str());
        label_text.push_str(format!("과목2: {}, 점수: {}\n", second_name, second_score).as_str());
    }
}