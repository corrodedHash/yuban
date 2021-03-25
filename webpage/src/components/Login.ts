
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
                this.$emit("login");
                this.checkedToken = false;
            } else {
                this.checkedToken = true;
            }
        },

        checkToken() {
            if (document.cookie.indexOf("token=") == -1) {
                this.checkedToken = true
                return;
            }
            check_token().then(this.handleLoginPromise.bind(this)).catch(() => {
                this.checkedToken = true;
            })
        },
        
        login() {
            check_login(this.username, this.password)
                .then(this.handleLoginPromise.bind(this)).catch(() => {
                    console.error("Login failed miserably");
                    this.checkedToken = true;
                }).finally(() => {
                    this.username = "";
                    this.password = "";
                });

        },
    },
});