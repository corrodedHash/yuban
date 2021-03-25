declare module 'diff';
interface Diff {
    diffWords(oldStr: string, newStr: string, options: any?): { value: string, added: boolean, removed: boolean }[]
}