
import { defineComponent, PropType } from "vue";
import { new_post, get_post, Post } from "@/components/api";
export default defineComponent({
    name: "PostEditor",
    props: {
        postid: { type: Number as PropType<number> },
    },
    data() {
        return { text: "" };
    },
    mounted() {
        this.handlePostChange()
    },
    watch: {
        postid(newPostID: number | undefined) {
            this.handlePostChange()
            console.log(newPostID)
            console.log(this.postid)
        }
    },
    computed: {
        canEdit(): boolean {
            return this.postid === undefined
        }
    },
    methods: {
        handlePostChange() {
            let me = this
            if (this.postid !== undefined) {
                get_post(this.postid, (req) => {
                    if (req.readyState == req.DONE && req.status >= 200 && req.status < 300) {
                        let post: Post = JSON.parse(req.responseText)
                        me.text = post.text
                    }
                })
            } else {
                this.text = ""
            }
        },
        handleSubmit() {
            if (this.postid !== undefined) {
                return
            }
            new_post(this.text, (this.$refs.langcode as any).value, (req) => {
                if (req.readyState === 4) {
                    if (req.status === 200) {
                    } else {
                        console.log("Could not post");
                    }
                }
            });
        },
    },
});