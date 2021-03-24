import { defineComponent } from "vue";
import { get_posts, ThreadSummary, PostSummary } from "./api";

export default defineComponent({
    name: "PostList",
    emits: {
        selectPost(thread_id: number | null, post_id: number | null) {
            return true;
        },
    },
    data() {
        return {
            posts: [] as ThreadSummary[],
        };
    },
    mounted() {
        this.requestPosts();
    },
    computed: {
        tableData(): ThreadSummary[] {
            return this.posts;
        },
    },
    methods: {
        selectNew() {
            this.$emit("selectPost", null, null);
            (this.$refs.postList as any).value = "";
        },
        selectNewPost(thread_id: number) {
            this.$emit("selectPost", thread_id, null);
        },
        handleSelectedPost(thread_index: number, index: PostSummary) {
            if (index !== null) {
                console.log("Selected: ", index);
                this.$emit("selectPost", thread_index, index.id);
            }
        },
        requestPosts() {
            let me = this;
            get_posts().then((posts) => {
                me.posts = posts;
            });
        },
    },
});