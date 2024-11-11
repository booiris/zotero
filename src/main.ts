import { createApp } from "vue";
import App from "./App.vue";
import router from './router'
import { is_login } from "./api/is_login";

const init = async () => {
    const app = createApp(App)

    const isLoggedIn = await is_login()
    const initialRoute = isLoggedIn ? '/main' : '/'
    router.push(initialRoute)

    app.use(router).mount("#app")
}

init()
