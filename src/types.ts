// Tipos globales para la app de portafolios

export interface EmisoraBusqueda {
  razon_social: string;
  emisoras: string;
  serie: string;
}

export interface IndiceItem {
  a: number;
  c: number;
  e: string;
  f: string;
  m: number;
  n: number;
  u: number;
  v: number;
  x: number;
  ytdp: number;
}

export interface IndicesType {
  SP500?: IndiceItem;
  FTSEBIVA?: IndiceItem;
  IPC?: IndiceItem;
  DJIA?: IndiceItem;
}

export interface ForexItem {
  c: number;
  m: number;
  u: number;
}

export interface ForexType {
  t: string;
  USDMXN?: ForexItem;
  EURMXN?: ForexItem;
}

export interface TopImporte {
  e: string;
  i: number;
  u: number;
}
export interface TopCambio {
  c: number;
  e: string;
  f: string;
  u: number;
}
export interface TopOperaciones {
  e: string;
  o: number;
  u: number;
}
export interface TopVolumen {
  e: string;
  i: number;
  u: number;
}
export interface TopType {
  importe: TopImporte[];
  bajan: TopCambio[];
  operaciones: TopOperaciones[];
  suben: TopCambio[];
  volumen: TopVolumen[];
}
