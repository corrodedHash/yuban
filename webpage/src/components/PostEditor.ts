
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
        if (this.postid !== undefined) {
            let me = this
            get_post(this.postid, (req) => {
                if (req.readyState == req.DONE && req.status >= 200 && req.status < 300) {
                    let post: Post = JSON.parse(req.responseText)
                    me.text = post.text
                }
            })
        }
    },
    watch: {
        postid(newPostID: number | undefined) {
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
        }
    },
    methods: {
        handleSubmit() {
            new_post(this.text, (req) => {
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