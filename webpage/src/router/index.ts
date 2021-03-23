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
    props: route => ({ postid: parseInt(route.params.postid as string) })
  }
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router
