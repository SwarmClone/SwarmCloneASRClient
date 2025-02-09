<script setup lang="ts">
import { Modal } from 'ant-design-vue';
import { ref, defineAsyncComponent } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { message } from 'ant-design-vue';
import { ExclamationCircleOutlined } from '@ant-design/icons-vue';
import { h } from 'vue';

import ConnectButton from './components/connection/ConnectButton.vue';
import MessageInput from './components/chat/MessageInput.vue';

const isConnected = ref(false);
const formOpen = ref(false);

const [modal] = Modal.useModal(); // 初始化 Modal

const handleConnectClick = () => {
  if (isConnected.value) {
    showDisconnectConfirm();
  } else {
    formOpen.value = true;
  }
};

const showDisconnectConfirm = () => {
  modal.confirm({
    title: '请确认您的操作',
    icon: h(ExclamationCircleOutlined),
    content: '您确定要取消连接吗？这将导致远程服务器无法获取到您的语音识别结果。',
    okText: '确定',
    okType: 'danger',
    cancelText: '取消',
    async onOk() {
      const result = await invoke("r_dis_connect_server");
      if (result === true) {
        isConnected.value = false;
        message.success('成功断开连接');
      } else {
        message.error('断开连接失败');
      }
    },
    onCancel() {
      console.log('继续连接');
    },
  });
};

const handleFormSubmit = async (formData: any) => {
  try {
    const result = await invoke("r_connect_server", { name: formData.name, server_address: formData.serverAddress, port: formData.port, login_code: formData.loginCode });
    if (result === true) {
      isConnected.value = true;
      formOpen.value = false;
      message.success('成功连接到服务器，现在可以开始使用了');
    } else {
      message.error('连接错误');
    }
  } catch (error) {
    console.error(error);
    message.error('连接失败');
  }
};

const ServerFormDrawer = defineAsyncComponent(() => import('./components/connection/ServerFormDrawer.vue'));
const MessageList = defineAsyncComponent(() => import('./components/chat/MessageList.vue'));
</script>

<template>
  <main class="container">
    <div class="connect-container">
      <ConnectButton 
        :is-connected="isConnected"
        @click="handleConnectClick"
      />
    </div>

    <div class="chat-container">
      <MessageList />
      <MessageInput />
    </div>

    <ServerFormDrawer
      :open="formOpen"
      @close="formOpen = false"
      @submit="handleFormSubmit"
      @update:isConnected="isConnected = $event"
    />
  </main>
</template>

<style scoped>
.container {
  display: grid;
  grid-template-columns: 1fr 4fr;
  height: 100vh;
}

.connect-container {
  padding: 20px;
}

.chat-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
}
</style>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  font-weight: 400;
  color: #0f0f0f;
  background-color: #f6f6f6;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
}
</style>