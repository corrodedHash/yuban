import { defineComponent } from "vue";
import { get_posts, ThreadSummary, PostSummary } from "./api";

export default defineComponent({
    name: "PostList",
    emits: {
        selectPost(thread_id: number | null, post_id: number | null) {
            return true;
        },
        selectCorrection(thread_id: number, post_id: number, corr_id: number | null) {
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
                this.$emit("selectPost", thread_index, index.id);
            }
        },
        selectNewCorrection(thread_id: number, post: PostSummary) {
            this.$emit("selectCorrection", thread_id, post.id, null);
        },
        handleSelectedCorrection(thread_index: number, index: PostSummary, corr_index: number) {
            this.$emit("selectCorrection", thread_index, index.id, corr_index);
        },
        requestPosts() {
            let me = this;
            get_posts().then((posts) => {
                me.posts = posts;
            });
        },
    },
});