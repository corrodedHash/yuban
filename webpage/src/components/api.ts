type XMLCallback = (req: XMLHttpRequest) => void;


export interface Post {
    id: number;
    posttime: string;
    text: string;
    user: string;
}

const API_ROOT = "api/"

export function check_login(username: string, password: string): Promise<boolean> {
    return new Promise((resolve, reject) => {
        let loginreq = new XMLHttpRequest();
        loginreq.responseType = "json"
        loginreq.onload = (ev) => {
            switch (loginreq.status) {
                case 200:
                    console.assert(typeof loginreq.response == "boolean")
                    resolve(loginreq.response)
                    return;
                default:
                    console.log("Unknown status", loginreq.status, loginreq.response)
                    reject()
            }
        };
        loginreq.onerror = (ev) => {
            console.log("Error login", ev)
            reject()
        }
        loginreq.onabort = (ev) => {
            console.log("Abort login", ev)
            reject()
        }
        let json_data = JSON.stringify({
            username: username,
            password: password,
        });
        loginreq.open("POST", `/${API_ROOT}logindata`, true);
        loginreq.send(json_data);
    })
}

export function check_token(): Promise<boolean> {
    return new Promise((resolve, reject) => {
        let loginreq = new XMLHttpRequest();
        loginreq.onload = (ev) => {
            switch (loginreq.status) {
                case 202:
                    resolve(true)
                    return;
                default:
                    console.log("Unknown status", loginreq.status, loginreq.response)
                    reject()
            }
        };
        loginreq.onerror = (ev) => {
            switch (loginreq.status) {
                case 401:
                    resolve(false)
                    return;
                default:
                    console.log("Unknown status", loginreq.status, loginreq.response)
                    reject()
            }
        }
        loginreq.onabort = (ev) => {
            console.log("Abort login", ev)
            reject()
        }
        loginreq.open("GET", `/${API_ROOT}testtoken`, true);
        loginreq.send();
    });
}

export function get_posts(): Promise<Object> {
    return new Promise((resolve, reject) => {
        let postreq = new XMLHttpRequest();
        postreq.onload = (ev) => {
            if (postreq.status === 200) {
                let x = JSON.parse(postreq.responseText);
                console.log(x)
                resolve(x)
            } else {
                reject()
            }
        };
        postreq.open("GET", `/${API_ROOT}posts`, true);
        postreq.send();
    })
}

export function get_post(post_id: number, callback: XMLCallback) {
    let postreq = new XMLHttpRequest();
    postreq.onreadystatechange = (ev) => {
        callback(postreq);
    };
    postreq.open("GET", `/${API_ROOT}post/${post_id}`, true);
    postreq.send();
}

export function new_post(post: string, langcode: string, callback: XMLCallback) {
    let postreq = new XMLHttpRequest();
    postreq.onreadystatechange = (ev) => {
        callback(postreq);
    };
    postreq.open("PUT", `/${API_ROOT}newpost/${langcode}`, true);
    postreq.send(post);
}