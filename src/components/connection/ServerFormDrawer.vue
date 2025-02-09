<script setup lang="ts">
import { reactive, ref } from 'vue';
import { Form, message } from 'ant-design-vue';
import type { Rule } from 'ant-design-vue/es/form';
import { invoke } from '@tauri-apps/api/core';

const formRef = ref();
const form = reactive({
  name: '',
  serverAddress: '',
  port: '',
  loginCode: ''
});

const rules: Record<string, Rule[]> = {
  name: [{ type: 'string', required: true, message: '必须输入名字', trigger: 'blur' }],
  serverAddress: [{ type: 'string', required: true, message: '必须输入服务器地址', trigger: 'blur' }],
  port: [{ type: 'string', required: true, message: '必须输入端口号', trigger: 'blur' }],
  loginCode: [{ type: 'string', required: true, message: '必须输入登录验证码', trigger: 'blur' }]
};

defineProps<{
  open: boolean;
}>();

const emit = defineEmits(['close', 'submit', 'update:isConnected']);

const validateForm = () => {
  return formRef.value.validate();
};

const onClose = () => {
  emit('close');
};

const onSubmit = async () => {
  try {
    await validateForm();
    const result = await invoke("r_connect_server", { name: form.name, server_address: form.serverAddress, port: form.port, login_code: form.loginCode });
    console.log(result);
    if (result === true) {
      emit('update:isConnected', true);
      onClose();
      message.success('成功连接到服务器，现在可以开始使用了');
    } else {
      message.error('连接错误');
    }
  } catch (error) {
    console.error(error);
  }
};
</script>

<template>
  <a-drawer
    title="连接服务器"
    :width="720"
    :open="open"
    :body-style="{ paddingBottom: '80px' }"
    :footer-style="{ textAlign: 'right' }"
    @close="$emit('close')"
  >
    <Form ref="formRef" :model="form" :rules="rules" layout="vertical">
      <a-row :gutter="16">
        <a-col :span="12">
          <Form.Item label="名字" name="name">
            <a-input v-model:value="form.name" placeholder="请输入名字" />
          </Form.Item>
        </a-col>
        <a-col :span="12">
          <Form.Item label="服务器地址" name="serverAddress">
            <a-input v-model:value="form.serverAddress" placeholder="请输入服务器地址" />
          </Form.Item>
        </a-col>
      </a-row>
      <a-row :gutter="16">
        <a-col :span="12">
          <Form.Item label="端口号" name="port">
            <a-input v-model:value="form.port" placeholder="请输入端口号" />
          </Form.Item>
        </a-col>
        <a-col :span="12">
          <Form.Item label="登录验证码" name="loginCode">
            <a-input-password v-model:value="form.loginCode" placeholder="请输入登录验证码" />
          </Form.Item>
        </a-col>
      </a-row>
    </Form>
    <template #extra>
      <a-space>
        <a-button @click="$emit('close')">取消</a-button>
        <a-button type="primary" @click="onSubmit">提交</a-button>
      </a-space>
    </template>
  </a-drawer>
</template>