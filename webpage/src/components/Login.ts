
import { defineComponent } from "vue";
import { check_token, check_login } from "./api";
export default defineComponent({
    name: "Login",
    emits: {
        login() {
            return true;
        },
    },
    data() {
        return {
            username: "",
            password: "",
            checkedToken: false,
        };
    },
    mounted() {
        this.checkToken();
    },
    methods: {
        handleLoginPromise(loggedIn: boolean) {
            if (loggedIn) {
                console.log("Accepted");
                this.$emit("login");
                this.checkedToken = false;
            } else {
                console.log("Not accepted");
                this.checkedToken = true;
            }
        },

        checkToken() {
            if (document.cookie.indexOf("token=") == -1) {
                console.log("No Token to test");
                this.checkedToken = true
                return;
            }
            check_token().then(this.handleLoginPromise.bind(this)).catch(() => {
                console.log("Token check errored");
                this.checkedToken = true;
            })
        },
        login() {
            let loginreq = new XMLHttpRequest();
            check_login(this.username, this.password)
                .then(this.handleLoginPromise.bind(this)).catch(() => {
                    console.log("Errored");
                    this.checkedToken = true;
                }).finally(() => {
                    this.username = "";
                    this.password = "";
                });

        },
    },
});