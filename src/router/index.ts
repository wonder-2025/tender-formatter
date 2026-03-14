import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => import('../views/Home.vue'),
    meta: { title: '首页' }
  },
  {
    path: '/format-confirm',
    name: 'FormatConfirm',
    component: () => import('../views/FormatConfirm.vue'),
    meta: { title: '格式确认' }
  },
  {
    path: '/preview',
    name: 'Preview',
    component: () => import('../views/Preview.vue'),
    meta: { title: '预览对比' }
  },
  {
    path: '/templates',
    name: 'Templates',
    component: () => import('../views/Templates.vue'),
    meta: { title: '模板管理' }
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () => import('../views/Settings.vue'),
    meta: { title: '设置' }
  },
  {
    path: '/about',
    name: 'About',
    component: () => import('../views/About.vue'),
    meta: { title: '关于' }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

router.beforeEach((to, _from, next) => {
  document.title = `${to.meta.title || '标书格式优化工具'} - 标书格式优化工具`
  next()
})

export default router
