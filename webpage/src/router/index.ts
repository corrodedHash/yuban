import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import PostEditor from '@/components/PostEditor.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'New',
    component: PostEditor,
    props: true
  },
  {
    path: '/post/:postid',
    name: 'View',
    component: PostEditor,
    props: route => ({ postid: parseInt(route.params.postid as string), threadid: undefined })
  },
  {
    path: '/correction/:postid/:corrid',
    name: 'Correction',
    component: PostEditor,
    props: route => ({ postid: parseInt(route.params.postid as string), corrid: parseInt(route.params.corrid as string) })
  },
  {
    path: '/newpost/:threadid',
    name: 'NewPost',
    component: PostEditor,
    props: route => ({ postid: undefined, threadid: parseInt(route.params.threadid as string) })
  },
  {
    path: '/newcorrection/:postid',
    name: 'NewCorrection',
    component: PostEditor,
    props: route => ({ postid: parseInt(route.params.postid as string), corrid: null })
  }
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router
