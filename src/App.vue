<script setup lang="ts"></script>

<template>
  <n-dialog-provider>
    <n-message-provider>
      <router-view v-slot="{ Component }">
        <transition name="fade" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </n-message-provider>
  </n-dialog-provider>
</template>

<style>
body {
  font-family: Arial, Helvetica, sans-serif;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>

<style></style>