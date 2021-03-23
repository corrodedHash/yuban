type XMLCallback = (req: XMLHttpRequest) => void;


export interface Post {
    id: number;
    posttime: string;
    text: string;
    user: string;
}

const API_ROOT = "api/"

export function check_login(username: string, password: string, callback: XMLCallback) {
    let loginreq = new XMLHttpRequest();
    loginreq.onreadystatechange = (ev) => {
        callback(loginreq);
    };
    let json_data = JSON.stringify({
        username: username,
        password: password,
    });
    loginreq.open("POST", `/${API_ROOT}logindata`, true);
    loginreq.send(json_data);
}

export function check_token(callback: XMLCallback) {
    let loginreq = new XMLHttpRequest();
    loginreq.onreadystatechange = (ev) => {
        callback(loginreq)
    };
    loginreq.open("GET", `/${API_ROOT}testtoken`, true);
    loginreq.send();
}

export function get_posts(callback: XMLCallback) {
    let postreq = new XMLHttpRequest();
    postreq.onreadystatechange = (ev) => {
        callback(postreq);
    };
    postreq.open("GET", `/${API_ROOT}posts`, true);
    postreq.send();
}

export function get_post(post_id: number, callback: XMLCallback) {
    let postreq = new XMLHttpRequest();
    postreq.onreadystatechange = (ev) => {
        callback(postreq);
    };
    postreq.open("GET", `/${API_ROOT}post/${post_id}`, true);
    postreq.send();
}

export function new_post(post: string, callback: XMLCallback) {
    let postreq = new XMLHttpRequest();
    postreq.onreadystatechange = (ev) => {
        callback(postreq);
    };
    postreq.open("PUT", `/${API_ROOT}newpost`, true);
    postreq.send(post);
}