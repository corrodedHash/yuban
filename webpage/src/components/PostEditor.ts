
import { defineComponent, PropType } from "vue";
import { new_post, get_post, Post, add_post } from "@/components/api";
export default defineComponent({
    name: "PostEditor",
    props: {
        postid: { type: Number as PropType<number> },
        threadid: { type: Number as PropType<number> },
    },
    data() {
        return { text: "" };
    },
    mounted() {
        this.handlePostChange()
    },
    watch: {
        postid(newPostID: [number, number | null] | undefined) {
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
            if (this.postid !== undefined) {
                let me = this
                get_post(this.postid).then((post) => {
                    me.text = post.text
                })
            } else {
                this.text = ""
            }
        },
        handleSubmit() {
            if (this.postid !== undefined) {
                return
            }
            let langcode = (this.$refs.langcode as any).value
            if (this.threadid === undefined) {
                new_post(this.text, langcode).catch(() => console.log("Could not post"))
            } else {
                add_post(this.text, this.threadid, langcode)
            }
        },
    },
});