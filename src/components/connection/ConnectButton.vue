<template> 
  <a-button 
    class="connect-button" 
    :type="props.isConnected ? 'primary' : 'dashed'" 
    shape="round" 
    :size="size" 
    @click="handleClick" 
  > 
    <template #icon> 
      <component :is="props.isConnected ? CheckCircleOutlined : DisconnectOutlined" /> 
    </template> 
    {{ props.isConnected ? '已连接' : '未连接' }} 
  </a-button> 
</template> 
   
<script setup lang="ts"> 
  import { CheckCircleOutlined, DisconnectOutlined } from '@ant-design/icons-vue'; 
  import type { SizeType } from 'ant-design-vue/es/config-provider'; 
   
  const props = defineProps<{ 
    isConnected: boolean; 
    size?: SizeType; 
  }>(); 
   
  const emit = defineEmits(['click']); 
   
  const handleClick = () => { 
    emit('click'); 
  }; 
</script> 
   
<style scoped> 
  .connect-button { 
    margin: 10px; 
  } 
</style> 