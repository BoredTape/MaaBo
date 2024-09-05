import { createRouter, createWebHistory } from 'vue-router'
import LoadingView from '../views/LoadingView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'Loading',
      component: LoadingView
    },
    {
      path: '/oneKey',
      name: 'oneKey',
      meta: {
        name: '一键长草'
      },
      component: () => import('../views/OneKeyView.vue')
    },
    {
      path: '/copilot',
      name: 'copilot',
      meta: {
        name: '自动战斗'
      },
      component: () => import('../views/CopilotView.vue')
    },
    {
      path: '/tools',
      name: 'tools',
      meta: {
        name: '小工具'
      },
      component: () => import('../views/ToolsView.vue')
    },
    {
      path: '/settings',
      name: 'settings',
      meta: {
        name: '设置'
      },
      component: () => import('../views/SettingsView.vue')
    }
  ]
})

export default router
