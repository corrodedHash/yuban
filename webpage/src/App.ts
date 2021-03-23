
import { defineComponent } from "vue";
import Login from "@/components/Login.vue";
import PostWindow from "@/components/PostWindow.vue";

export default defineComponent({
  name: "App",
  components: { Login, PostWindow },
  data() {
    return { loggedIn: false }
  }
});