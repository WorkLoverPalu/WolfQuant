<template>
  <div class="tab-item" :class="{ 'active': active }" @click="$emit('click')">
    <div class="tab-content">
      <span class="tab-title">{{ tab.title }}</span>
      <span v-if="tab.props?.price" class="tab-price">{{ tab.props.price }}</span>
      <span v-if="tab.props?.change" class="tab-change" :class="getChangeClass(tab.props.change)">
        {{ tab.props.change }}
      </span>
    </div>
    <button v-if="tab.closable" class="close-button" @click.stop="$emit('close')">
      <CloseIcon />
    </button>
  </div>
</template>

<script setup lang="ts">
import CloseIcon from '../../assets/icons/CloseIcon.vue';

interface TabProps {
  id: string;
  title: string;
  component: string;
  props?: Record<string, any>;
  closable: boolean;
}

defineProps<{
  tab: TabProps;
  active: boolean;
}>();

defineEmits<{
  (e: 'click'): void;
  (e: 'close'): void;
}>();

const getChangeClass = (change: string) => {
  if (change.startsWith('+')) return 'positive';
  if (change.startsWith('-')) return 'negative';
  return '';
};
</script>

<style lang="scss" scoped>
.tab-item {
  display: flex;
  align-items: center;
  height: 100%;
  padding: 0 var(--spacing-md);
  min-width: 120px;
  max-width: 200px;
  background-color: var(--cardBg);
  color: var(--tabText);
  border-right: 1px solid var(--borderColor);
  cursor: pointer;
  user-select: none;
  position: relative;
  transition: background-color var(--transition-fast), color var(--transition-fast);

  &.active {
    background-color: var(--accentColor);
    color: var(--tabActiveText);
  }

  &:hover {
    .close-button {
      opacity: 1;
    }
  }
}

.tab-content {
  display: flex;
  align-items: center;
  gap: 6px;
  flex: 1;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.tab-title {
  font-size: 13px;
  font-weight: 500;
}

.tab-price {
  font-size: var(--font-size-sm);
}

.tab-change {
  font-size: var(--font-size-sm);

  &.positive {
    color: var(--positiveColor);
  }

  &.negative {
    color: var(--negativeColor);
  }
}

.close-button {
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: inherit;
  opacity: 0.5;
  padding: 2px;
  margin-left: 6px;
  border-radius: 50%;
  transition: background-color var(--transition-fast);

  &:hover {
    background-color: var(--hoverBg);
  }
}
</style>