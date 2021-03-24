type XMLCallback = (req: XMLHttpRequest) => void;


export interface ThreadSummary {
    id: number;
    posttime: string;
    posts: string;
    user: string;
}

export interface PostSummary {
    id: number,
    date: string,
    ellipsis: string,
    user: string,
    lang: string,
    corrections: string

}

export interface Post {
    thread_id: number,
    id: number,
    posttime: string,
    langcode: string,
    correction_for: number | null,
    text: string
}

const API_ROOT = "/api"

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
        loginreq.open("POST", `${API_ROOT}/logindata`, true);
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
        loginreq.open("GET", `${API_ROOT}/testtoken`, true);
        loginreq.send();
    });
}

export function get_posts(): Promise<ThreadSummary[]> {
    return new Promise((resolve, reject) => {
        let postreq = new XMLHttpRequest();
        postreq.onload = (ev) => {
            if (postreq.status === 200) {
                let x = JSON.parse(postreq.responseText) as any[];
                let threads: ThreadSummary[] = x.map(thread => {
                    thread.posts = JSON.parse(thread.posts)
                    return thread
                })
                resolve(threads)
            } else {
                reject()
            }
        };
        postreq.open("GET", `${API_ROOT}/posts`, true);
        postreq.send();
    })
}

export function get_post(post_id: number): Promise<Post> {
    return new Promise((resolve, reject) => {
        let postreq = new XMLHttpRequest();
        postreq.responseType = "json"
        postreq.onload = (ev) => {
            if (postreq.status === 200) {
                resolve(postreq.response)
            } else {
                reject()
            }
        };
        postreq.open("GET", `${API_ROOT}/post/${post_id}`, true);
        postreq.send();
    })
}

export function new_post(post: string, langcode: string): Promise<void> {
    return new Promise((resolve, reject) => {
        let postreq = new XMLHttpRequest();
        postreq.onload = () => {
            resolve()
        }
        postreq.onerror = () => {
            reject()
        }
        postreq.onabort = () => {
            reject()
        }
        postreq.open("PUT", `${API_ROOT}/newpost/${langcode}`, true);
        postreq.send(post);
    })
}

export function add_post(post: string, thread_id: number, langcode: string): Promise<void> {
    return new Promise((resolve, reject) => {
        let postreq = new XMLHttpRequest();
        postreq.onload = () => {
            resolve()
        }
        postreq.onerror = () => {
            reject()
        }
        postreq.onabort = () => {
            reject()
        }
        postreq.open("PUT", `${API_ROOT}/addpost/${thread_id}/${langcode}`, true);
        postreq.send(post);
    })
}