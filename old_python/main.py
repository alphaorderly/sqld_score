import tkinter as tk
from tkinter import messagebox
from requests import post, get
from time import sleep
import requests

class TestResultApp:
    def __init__(self, root):
        self.root = root
        self.root.title("SQLD 테스트 결과 미리 확인 프로그램")

        self.id_label = tk.Label(root, text="dataq 아이디:")
        self.id_label.grid(row=0, column=0, padx=10, pady=10)

        self.id_entry = tk.Entry(root)
        self.id_entry.grid(row=0, column=1, padx=10, pady=10)

        self.pw_label = tk.Label(root, text="dataq 비밀번호:")
        self.pw_label.grid(row=1, column=0, padx=10, pady=10)

        self.pw_entry = tk.Entry(root, show="*")
        self.pw_entry.grid(row=1, column=1, padx=10, pady=10)

        self.check_button = tk.Button(root, text="결과 확인하기", command=self.check_results)
        self.check_button.grid(row=2, columnspan=2, pady=10)

        self.result_text = tk.Text(root, state='disabled', width=80, height=20)
        self.result_text.grid(row=3, columnspan=2, padx=10, pady=10)

    def check_results(self):
        user_id = self.id_entry.get()
        user_pw = self.pw_entry.get()
        if not user_id or not user_pw:
            messagebox.showwarning("Input Error", "아이디와 비밀번호를 둘 다 입력해주세요. dataq.or.kr 의 아이디와 비밀번호입니다.")
            return

        try:
            url = "https://www.dataq.or.kr/www/accounts/login/proc.do"
            payload = f'userperm=S01&loginid={user_id}&loginpw={user_pw}'
            headers = {'Content-Type': 'application/x-www-form-urlencoded'}
            login = post(url, headers=headers, data=payload, allow_redirects=False)

            if 'JSESSIONID' not in login.cookies:
                messagebox.showerror("Login Error", "로그인에 실패했습니다.")
                return

            sessionID = login.cookies.get_dict()['JSESSIONID']
            sleep(2)

            url = "https://www.dataq.or.kr/www/mypage/accept/result.dox"
            headers = {'Content-Type': 'application/json; charset=UTF-8'}
            response = requests.request("POST", url, headers=headers, data="{}", cookies={'JSESSIONID': sessionID})

            testList = response.json()['list']
            tests = []

            for test in testList:
                aplyseq = test['aplySeq']
                testData = post(
                    'https://www.dataq.or.kr/www/mypage/accept/score.dox',
                    headers={'Content-Type': 'application/json;charset=UTF-8'},
                    json={"aplySeq": aplyseq},
                    cookies={'JSESSIONID': sessionID}
                )
                tests.append(testData.json())

            self.result_text.config(state='normal')
            self.result_text.delete(1.0, tk.END)
            
            self.result_text.insert(tk.END, f"채점이 되지 않았을시 0점으로 출력됩니다.\n\n")
            
            self.result_text.insert(tk.END, f"총 {len(tests)}개의 시험 결과가 있습니다.\n\n")

            for td in tests:
                testData = td['info']
                result = f"{testData['examdatetimeSt']} 에 실시한 {testData['examnm']}의 총 점수는 {testData['hitpoint']} 점입니다.\n"
                result += f"1과목 : {testData['lecturenm1']} - {testData['lectureHitpoint1']} 점\n"
                result += f"2과목 : {testData['lecturenm2']} - {testData['lectureHitpoint2']} 점\n"
                firstScore = testData['lectureHitpoint1']
                secondScore = testData['lectureHitpoint2']
                fullScore = testData['hitpoint']
                if firstScore >= 40 and secondScore >= 40 and fullScore >= 60:
                    result += '합격입니다.\n'
                else:
                    result += '불합격입니다.\n'
                self.result_text.insert(tk.END, result + "\n")

            self.result_text.config(state='disabled')

        except Exception as e:
            messagebox.showerror("Error", str(e))

if __name__ == "__main__":
    root = tk.Tk()
    app = TestResultApp(root)
    root.mainloop()
