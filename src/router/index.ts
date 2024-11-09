import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'login',
            component: () => import('@/views/loginView.vue')
        },
        {
            path: '/main',
            name: 'main',
            component: () => import('@/views/MainView.vue'),
            children: [
                {
                    path: 'collection/:key',
                    name: 'collection',
                    component: () => import('@/components/ItemListView.vue')
                }
            ]
        }
    ]
})

export default router
