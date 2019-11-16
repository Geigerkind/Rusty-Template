export class AccountInformation {
  constructor(
    public readonly id: number,
    public readonly mail: string,
    public readonly mail_confirmed: boolean,
    public readonly nickname: string
  ) {}
}
