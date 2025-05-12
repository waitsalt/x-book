import type { RouteRecordRaw } from "vue-router";
import HomeView from "./view/HomeView.vue";
import { route as userRoute } from "./user/route";

const route: RouteRecordRaw[] = [
  {
    path: "/",
    name: "网站首页",
    component: HomeView,
  },
  ...userRoute,
];

export { route };
