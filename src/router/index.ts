import { createRouter, createWebHistory } from 'vue-router'
import { is_login } from '@/api/is_login'

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


router.beforeEach(async (to, _, next) => {
    const isLoggedIn = await is_login()

    if (to.name !== 'login' && !isLoggedIn) {
        next({ name: 'login' })
    } else if (to.name === 'login' && isLoggedIn) {
        next({ name: 'main' })
    } else {
        next()
    }
})

export default router
