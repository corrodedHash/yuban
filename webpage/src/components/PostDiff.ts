
import { defineComponent, PropType } from "vue";
import { diffWords } from "diff";


export default defineComponent({
    name: "PostDiff",
    props: {
        original: { type: String, required: true },
        correction: { type: String, required: true },
    },
    computed: {
        difftext(): { text: string, type: "normal" | "add" | "sub" }[] {
            let diffs = (diffWords(this.original, this.correction) as { value: string, added: boolean, removed: boolean }[]);
            let id = 0;
            let mapped_diffs = diffs.map((v) => {
                id += 1
                return { id: id, text: v.value, type: (v.added ? "add" : (v.removed ? "sub" : "normal")) as "normal" | "add" | "sub" }
            })
            return mapped_diffs;
        }
    },
});