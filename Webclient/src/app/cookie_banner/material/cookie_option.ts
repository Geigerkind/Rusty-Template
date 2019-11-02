
export class CookieOption {
  constructor(
    public readonly title: string,
    public readonly description: string,
    private _enabled: boolean,
    public readonly disabled: boolean
  ) {}

  public get enabled(): boolean {
    return this._enabled;
  }

  public toggle() {
    if (this.disabled)
      return;
    this._enabled = !this._enabled;
  }
}
