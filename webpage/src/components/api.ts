type XMLCallback = (req: XMLHttpRequest) => void;


export interface Post {
    id: number;
    posttime: string;
    text: string;
    user: string;
}

export function check_login(username: string, password: string, callback: XMLCallback) {
    let loginreq = new XMLHttpRequest();
    loginreq.onreadystatechange = (ev) => {
        callback(loginreq);
    };
    let json_data = JSON.stringify({
        username: username,
        password: password,
    });
    loginreq.open("POST", "/logindata", true);
    loginreq.send(json_data);
}

export function check_token(callback: XMLCallback) {
    let loginreq = new XMLHttpRequest();
    loginreq.onreadystatechange = (ev) => {
        callback(loginreq)
    };
    loginreq.open("GET", "/testtoken", true);
    loginreq.send();
}

export function get_posts(callback: XMLCallback) {
    let postreq = new XMLHttpRequest();
    postreq.onreadystatechange = (ev) => {
        callback(postreq);
    };
    postreq.open("GET", "/posts", true);
    postreq.send();
}

export function get_post(post_id: number, callback: XMLCallback) {
    let postreq = new XMLHttpRequest();
    postreq.onreadystatechange = (ev) => {
        callback(postreq);
    };
    postreq.open("GET", `/post/${post_id}`, true);
    postreq.send();
}

export function new_post(post: string, callback: XMLCallback) {
    let postreq = new XMLHttpRequest();
    postreq.onreadystatechange = (ev) => {
        callback(postreq);
    };
    postreq.open("PUT", "/newpost", true);
    postreq.send(post);
}