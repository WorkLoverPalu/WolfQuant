export interface User {
    id: string;
    username: string;
    email: string;
    avatar?: string;
  }

  export interface Tab {
    id: string;
    title: string;
    component: any;
    props?: Record<string, any>;
    closable: boolean;
  }

  // 市场数据类型
  export interface MarketData {
    name: string;
    symbol: string;
    price: string;
    change: string;
    percentChange: string;
    high: string;
    low: string;
    open: string;
    prevClose: string;
    volume: string;
    updateTime: string;
  }