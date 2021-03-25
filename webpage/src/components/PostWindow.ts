
import { defineComponent } from "vue";
import PostEditor from "@/components/PostEditor.vue";
import PostList from "@/components/PostList.vue";

export default defineComponent({
    name: "PostWindow",
    components: { PostList, PostEditor },
    emits: {
        logout() {
            return true
        }
    },
    methods: {
        openList() {
            this.$router.push({ name: "Menu" })
        },
        handleLogout() {
            document.cookie = "token=;expires=Thu, 01-Jan-1970 00:00:00 GMT"
            document.cookie = "username=;expires=Thu, 01-Jan-1970 00:00:00 GMT"
            this.$emit('logout')
        },
        handleCorrectionSwitch(thread_id: number, post_id: number, correction: number | null) {
            if (correction !== null) {
                this.$router.push({ name: "Correction", params: { postid: post_id, corrid: correction } })
            } else {
                this.$router.push({ name: "NewCorrection", params: { postid: post_id } })
            }
        },
        handlePostSwitch(thread_id: number | null, post_id: number | null) {
            if (post_id === null && thread_id === null) {
                this.$router.push({ path: '/' })
            } else if (post_id !== null && thread_id === null) {
            } else if (post_id !== null && thread_id !== null) {
                this.$router.push({ name: 'View', params: { postid: post_id } })
            } else if (post_id === null && thread_id !== null) {
                this.$router.push({ name: "NewPost", params: { threadid: thread_id } })
            }
        }
    }
});