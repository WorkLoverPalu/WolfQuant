<template>
    <div v-if="show" class="confirm-dialog-overlay" @click.self="$emit('cancel')">
        <div class="confirm-dialog">
            <div class="confirm-dialog-header">
                <h3>{{ title }}</h3>
                <button class="close-button" @click="$emit('cancel')">
                    <XIcon />
                </button>
            </div>
            <div class="confirm-dialog-content">
                <p>{{ message }}</p>
            </div>
            <div class="confirm-dialog-actions">
                <button class="cancel-button" @click="$emit('cancel')">
                    {{ cancelText }}
                </button>
                <button class="confirm-button" @click="$emit('confirm')" :class="{ 'danger': danger }">
                    {{ confirmText }}
                </button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { XIcon } from 'lucide-vue-next';

defineProps({
    show: {
        type: Boolean,
        required: true
    },
    title: {
        type: String,
        default: '确认'
    },
    message: {
        type: String,
        required: true
    },
    confirmText: {
        type: String,
        default: '确认'
    },
    cancelText: {
        type: String,
        default: '取消'
    },
    danger: {
        type: Boolean,
        default: false
    }
});

defineEmits(['confirm', 'cancel']);
</script>

<style lang="scss" scoped>
.confirm-dialog-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1100;
}

.confirm-dialog {
    background-color: var(--cardBg);
    border-radius: 8px;
    width: 90%;
    max-width: 400px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
    overflow: hidden;
}

.confirm-dialog-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px;
    border-bottom: 1px solid var(--borderColor);

    h3 {
        margin: 0;
        font-size: 18px;
        font-weight: 600;
        color: var(--textColor);
    }

    .close-button {
        background: transparent;
        border: none;
        color: var(--textSecondary);
        cursor: pointer;
        padding: 4px;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 4px;

        &:hover {
            background-color: var(--hover-bg);
            color: var(--textColor);
        }

        svg {
            width: 20px;
            height: 20px;
        }
    }
}

.confirm-dialog-content {
    padding: 20px;
    color: var(--textColor);

    p {
        margin: 0;
        line-height: 1.5;
    }
}

.confirm-dialog-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    padding: 16px 20px;
    border-top: 1px solid var(--borderColor);
}

.cancel-button,
.confirm-button {
    padding: 8px 16px;
    border-radius: 4px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s;
}

.cancel-button {
    background-color: transparent;
    border: 1px solid var(--borderColor);
    color: var(--textColor);

    &:hover {
        background-color: var(--hover-bg);
    }
}

.confirm-button {
    background-color: var(--accentColor);
    border: none;
    color: white;

    &:hover {
        background-color: var(--accentColorHover);
    }

    &.danger {
        background-color: var(--negativeColor);

        &:hover {
            background-color: var(--negativeColorHover, #d32f2f);
        }
    }
}
</style>