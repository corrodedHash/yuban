
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
    data() {
        return { postid: undefined as number | undefined }
    },
    methods: {
        handleLogout() {
            document.cookie = "token=;expires=Thu, 01-Jan-1970 00:00:00 GMT"
            document.cookie = "username=;expires=Thu, 01-Jan-1970 00:00:00 GMT"
            this.$emit('logout')
        },
        handlePostSwitch(postid: null | number) {
            if (postid === null) {
                this.postid = undefined
                this.$router.push({ path: '/' })
            } else {
                this.postid = postid
                this.$router.push({ name: 'View', params: { postid: this.postid } })
            }
        }
    }
});