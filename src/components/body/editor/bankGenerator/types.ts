export enum BankType {
    Crypt = "BTD_BANK_CRYPT",
    Pyramid = "BTD_BANK_PYRAMID",
    MagiVault = "BTD_BANK_MAGI_VAULT",
    DragonUtopia = "BTD_BANK_DRAGON_UTOPIA",
    ElementalStockpile = "BTD_BANK_ELEMENTAL_STOCKPILE",
    DwarvenTreasure = "BTD_BANK_DWARVEN_TREASURE",
    BloodTemple = "BTD_BANK_BLOOD_TEMPLE",
    TreantThicket = "BTD_BANK_TREANT_THICKET",
    GargoyleStonevault = "BTD_BANK_GARGOYLE_STONEVAULT",
    SunkenTemple = "BTD_BANK_SUNKEN_TEMPLE"
}

export type BankLoadingModel = {
    id: number,
    name: string
}

export type BankFullModel = {
    id: number,
    name: string,
    type: BankType,
    recharge_count: number,
    recharge_timer: number,
    luck_loss: number,
    morale_loss: number
}