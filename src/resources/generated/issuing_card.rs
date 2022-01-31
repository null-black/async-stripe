// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::ids::IssuingCardId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{
    Address, CardBrand, Currency, IssuingCardShippingStatus, IssuingCardShippingType,
    IssuingCardType, IssuingCardholder, MerchantCategory,
};

/// The resource representing a Stripe "IssuingCard".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCard {
    /// Unique identifier for the object.
    pub id: IssuingCardId,

    /// The brand of the card.
    pub brand: CardBrand,

    /// The reason why the card was canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<Box<IssuingCardCancellationReason>>,

    pub cardholder: IssuingCardholder,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The card's CVC.
    ///
    /// For security reasons, this is only available for virtual cards, and will be omitted unless you explicitly request it with [the `expand` parameter](https://stripe.com/docs/api/expanding_objects).
    /// Additionally, it's only available via the ["Retrieve a card" endpoint](https://stripe.com/docs/api/issuing/cards/retrieve), not via "List all cards" or any other endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<Box<String>>,

    /// The expiration month of the card.
    pub exp_month: i64,

    /// The expiration year of the card.
    pub exp_year: i64,

    /// The last 4 digits of the card number.
    pub last4: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The full unredacted card number.
    ///
    /// For security reasons, this is only available for virtual cards, and will be omitted unless you explicitly request it with [the `expand` parameter](https://stripe.com/docs/api/expanding_objects).
    /// Additionally, it's only available via the ["Retrieve a card" endpoint](https://stripe.com/docs/api/issuing/cards/retrieve), not via "List all cards" or any other endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<Box<String>>,

    /// The latest card that replaces this card, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_by: Option<Box<Expandable<IssuingCard>>>,

    /// The card this card replaces, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_for: Option<Box<Expandable<IssuingCard>>>,

    /// The reason why the previous card needed to be replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_reason: Option<Box<IssuingCardReplacementReason>>,

    /// Where and how the card will be shipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Box<IssuingCardShipping>>,

    pub spending_controls: IssuingCardAuthorizationControls,

    /// Whether authorizations can be approved on this card.
    pub status: IssuingCardStatus,

    /// The type of the card.
    #[serde(rename = "type")]
    pub type_: IssuingCardType,

    /// Information relating to digital wallets (like Apple Pay and Google Pay).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallets: Option<Box<IssuingCardWallets>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardAuthorizationControls {
    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to allow.
    ///
    /// All other categories will be blocked.
    /// Cannot be set with `blocked_categories`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_categories: Option<Vec<MerchantCategory>>,

    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to decline.
    ///
    /// All other categories will be allowed.
    /// Cannot be set with `allowed_categories`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_categories: Option<Vec<MerchantCategory>>,

    /// Limit spending with amount-based rules that apply across any cards this card replaced (i.e., its `replacement_for` card and _that_ card's `replacement_for` card, up the chain).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits: Option<Box<Vec<IssuingCardSpendingLimit>>>,

    /// Currency of the amounts within `spending_limits`.
    ///
    /// Always the same as the currency of the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits_currency: Option<Currency>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardShipping {
    pub address: Address,

    /// The delivery company that shipped a card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<Box<IssuingCardShippingCarrier>>,

    /// A unix timestamp representing a best estimate of when the card will be delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eta: Option<Box<Timestamp>>,

    /// Recipient name.
    pub name: String,

    /// Shipment service, such as `standard` or `express`.
    pub service: IssuingCardShippingService,

    /// The delivery status of the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<IssuingCardShippingStatus>,

    /// A tracking number for a card shipment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<Box<String>>,

    /// A link to the shipping carrier's site where you can view detailed information about a card shipment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<Box<String>>,

    /// Packaging options.
    #[serde(rename = "type")]
    pub type_: IssuingCardShippingType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardSpendingLimit {
    /// Maximum amount allowed to spend per interval.
    pub amount: i64,

    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) this limit applies to.
    ///
    /// Omitting this field will apply the limit to all categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Box<Vec<IssuingCardSpendingLimitCategories>>>,

    /// Interval (or event) to which the amount applies.
    pub interval: IssuingCardSpendingLimitInterval,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardWallets {
    pub apple_pay: IssuingCardApplePay,

    pub google_pay: IssuingCardGooglePay,

    /// Unique identifier for a card used with digital wallets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_account_identifier: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardApplePay {
    /// Apple Pay Eligibility.
    pub eligible: bool,

    /// Reason the card is ineligible for Apple Pay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ineligible_reason: Option<Box<IssuingCardApplePayIneligibleReason>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardGooglePay {
    /// Google Pay Eligibility.
    pub eligible: bool,

    /// Reason the card is ineligible for Google Pay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ineligible_reason: Option<Box<IssuingCardGooglePayIneligibleReason>>,
}

/// An enum representing the possible values of an `IssuingCardApplePay`'s `ineligible_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardApplePayIneligibleReason {
    MissingAgreement,
    MissingCardholderContact,
    UnsupportedRegion,
}

impl IssuingCardApplePayIneligibleReason {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardApplePayIneligibleReason::MissingAgreement => "missing_agreement",
            IssuingCardApplePayIneligibleReason::MissingCardholderContact => {
                "missing_cardholder_contact"
            }
            IssuingCardApplePayIneligibleReason::UnsupportedRegion => "unsupported_region",
        }
    }
}

impl AsRef<str> for IssuingCardApplePayIneligibleReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardApplePayIneligibleReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCard`'s `cancellation_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardCancellationReason {
    Lost,
    Stolen,
}

impl IssuingCardCancellationReason {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardCancellationReason::Lost => "lost",
            IssuingCardCancellationReason::Stolen => "stolen",
        }
    }
}

impl AsRef<str> for IssuingCardCancellationReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCardGooglePay`'s `ineligible_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardGooglePayIneligibleReason {
    MissingAgreement,
    MissingCardholderContact,
    UnsupportedRegion,
}

impl IssuingCardGooglePayIneligibleReason {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardGooglePayIneligibleReason::MissingAgreement => "missing_agreement",
            IssuingCardGooglePayIneligibleReason::MissingCardholderContact => {
                "missing_cardholder_contact"
            }
            IssuingCardGooglePayIneligibleReason::UnsupportedRegion => "unsupported_region",
        }
    }
}

impl AsRef<str> for IssuingCardGooglePayIneligibleReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardGooglePayIneligibleReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCard`'s `replacement_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardReplacementReason {
    Damaged,
    Expired,
    Lost,
    Stolen,
}

impl IssuingCardReplacementReason {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardReplacementReason::Damaged => "damaged",
            IssuingCardReplacementReason::Expired => "expired",
            IssuingCardReplacementReason::Lost => "lost",
            IssuingCardReplacementReason::Stolen => "stolen",
        }
    }
}

impl AsRef<str> for IssuingCardReplacementReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardReplacementReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCardShipping`'s `carrier` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardShippingCarrier {
    Dhl,
    Fedex,
    RoyalMail,
    Usps,
}

impl IssuingCardShippingCarrier {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardShippingCarrier::Dhl => "dhl",
            IssuingCardShippingCarrier::Fedex => "fedex",
            IssuingCardShippingCarrier::RoyalMail => "royal_mail",
            IssuingCardShippingCarrier::Usps => "usps",
        }
    }
}

impl AsRef<str> for IssuingCardShippingCarrier {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardShippingCarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCardShipping`'s `service` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardShippingService {
    Express,
    Priority,
    Standard,
}

impl IssuingCardShippingService {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardShippingService::Express => "express",
            IssuingCardShippingService::Priority => "priority",
            IssuingCardShippingService::Standard => "standard",
        }
    }
}

impl AsRef<str> for IssuingCardShippingService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardShippingService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCardSpendingLimit`'s `categories` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardSpendingLimitCategories {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    #[serde(rename = "heating_plumbing_a_c")]
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    Miscellaneous,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    #[serde(rename = "u_s_federal_government_agencies_or_departments")]
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
}

impl IssuingCardSpendingLimitCategories {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardSpendingLimitCategories::AcRefrigerationRepair => "ac_refrigeration_repair",
            IssuingCardSpendingLimitCategories::AccountingBookkeepingServices => "accounting_bookkeeping_services",
            IssuingCardSpendingLimitCategories::AdvertisingServices => "advertising_services",
            IssuingCardSpendingLimitCategories::AgriculturalCooperative => "agricultural_cooperative",
            IssuingCardSpendingLimitCategories::AirlinesAirCarriers => "airlines_air_carriers",
            IssuingCardSpendingLimitCategories::AirportsFlyingFields => "airports_flying_fields",
            IssuingCardSpendingLimitCategories::AmbulanceServices => "ambulance_services",
            IssuingCardSpendingLimitCategories::AmusementParksCarnivals => "amusement_parks_carnivals",
            IssuingCardSpendingLimitCategories::AntiqueReproductions => "antique_reproductions",
            IssuingCardSpendingLimitCategories::AntiqueShops => "antique_shops",
            IssuingCardSpendingLimitCategories::Aquariums => "aquariums",
            IssuingCardSpendingLimitCategories::ArchitecturalSurveyingServices => "architectural_surveying_services",
            IssuingCardSpendingLimitCategories::ArtDealersAndGalleries => "art_dealers_and_galleries",
            IssuingCardSpendingLimitCategories::ArtistsSupplyAndCraftShops => "artists_supply_and_craft_shops",
            IssuingCardSpendingLimitCategories::AutoAndHomeSupplyStores => "auto_and_home_supply_stores",
            IssuingCardSpendingLimitCategories::AutoBodyRepairShops => "auto_body_repair_shops",
            IssuingCardSpendingLimitCategories::AutoPaintShops => "auto_paint_shops",
            IssuingCardSpendingLimitCategories::AutoServiceShops => "auto_service_shops",
            IssuingCardSpendingLimitCategories::AutomatedCashDisburse => "automated_cash_disburse",
            IssuingCardSpendingLimitCategories::AutomatedFuelDispensers => "automated_fuel_dispensers",
            IssuingCardSpendingLimitCategories::AutomobileAssociations => "automobile_associations",
            IssuingCardSpendingLimitCategories::AutomotivePartsAndAccessoriesStores => "automotive_parts_and_accessories_stores",
            IssuingCardSpendingLimitCategories::AutomotiveTireStores => "automotive_tire_stores",
            IssuingCardSpendingLimitCategories::BailAndBondPayments => "bail_and_bond_payments",
            IssuingCardSpendingLimitCategories::Bakeries => "bakeries",
            IssuingCardSpendingLimitCategories::BandsOrchestras => "bands_orchestras",
            IssuingCardSpendingLimitCategories::BarberAndBeautyShops => "barber_and_beauty_shops",
            IssuingCardSpendingLimitCategories::BettingCasinoGambling => "betting_casino_gambling",
            IssuingCardSpendingLimitCategories::BicycleShops => "bicycle_shops",
            IssuingCardSpendingLimitCategories::BilliardPoolEstablishments => "billiard_pool_establishments",
            IssuingCardSpendingLimitCategories::BoatDealers => "boat_dealers",
            IssuingCardSpendingLimitCategories::BoatRentalsAndLeases => "boat_rentals_and_leases",
            IssuingCardSpendingLimitCategories::BookStores => "book_stores",
            IssuingCardSpendingLimitCategories::BooksPeriodicalsAndNewspapers => "books_periodicals_and_newspapers",
            IssuingCardSpendingLimitCategories::BowlingAlleys => "bowling_alleys",
            IssuingCardSpendingLimitCategories::BusLines => "bus_lines",
            IssuingCardSpendingLimitCategories::BusinessSecretarialSchools => "business_secretarial_schools",
            IssuingCardSpendingLimitCategories::BuyingShoppingServices => "buying_shopping_services",
            IssuingCardSpendingLimitCategories::CableSatelliteAndOtherPayTelevisionAndRadio => "cable_satellite_and_other_pay_television_and_radio",
            IssuingCardSpendingLimitCategories::CameraAndPhotographicSupplyStores => "camera_and_photographic_supply_stores",
            IssuingCardSpendingLimitCategories::CandyNutAndConfectioneryStores => "candy_nut_and_confectionery_stores",
            IssuingCardSpendingLimitCategories::CarAndTruckDealersNewUsed => "car_and_truck_dealers_new_used",
            IssuingCardSpendingLimitCategories::CarAndTruckDealersUsedOnly => "car_and_truck_dealers_used_only",
            IssuingCardSpendingLimitCategories::CarRentalAgencies => "car_rental_agencies",
            IssuingCardSpendingLimitCategories::CarWashes => "car_washes",
            IssuingCardSpendingLimitCategories::CarpentryServices => "carpentry_services",
            IssuingCardSpendingLimitCategories::CarpetUpholsteryCleaning => "carpet_upholstery_cleaning",
            IssuingCardSpendingLimitCategories::Caterers => "caterers",
            IssuingCardSpendingLimitCategories::CharitableAndSocialServiceOrganizationsFundraising => "charitable_and_social_service_organizations_fundraising",
            IssuingCardSpendingLimitCategories::ChemicalsAndAlliedProducts => "chemicals_and_allied_products",
            IssuingCardSpendingLimitCategories::ChildCareServices => "child_care_services",
            IssuingCardSpendingLimitCategories::ChildrensAndInfantsWearStores => "childrens_and_infants_wear_stores",
            IssuingCardSpendingLimitCategories::ChiropodistsPodiatrists => "chiropodists_podiatrists",
            IssuingCardSpendingLimitCategories::Chiropractors => "chiropractors",
            IssuingCardSpendingLimitCategories::CigarStoresAndStands => "cigar_stores_and_stands",
            IssuingCardSpendingLimitCategories::CivicSocialFraternalAssociations => "civic_social_fraternal_associations",
            IssuingCardSpendingLimitCategories::CleaningAndMaintenance => "cleaning_and_maintenance",
            IssuingCardSpendingLimitCategories::ClothingRental => "clothing_rental",
            IssuingCardSpendingLimitCategories::CollegesUniversities => "colleges_universities",
            IssuingCardSpendingLimitCategories::CommercialEquipment => "commercial_equipment",
            IssuingCardSpendingLimitCategories::CommercialFootwear => "commercial_footwear",
            IssuingCardSpendingLimitCategories::CommercialPhotographyArtAndGraphics => "commercial_photography_art_and_graphics",
            IssuingCardSpendingLimitCategories::CommuterTransportAndFerries => "commuter_transport_and_ferries",
            IssuingCardSpendingLimitCategories::ComputerNetworkServices => "computer_network_services",
            IssuingCardSpendingLimitCategories::ComputerProgramming => "computer_programming",
            IssuingCardSpendingLimitCategories::ComputerRepair => "computer_repair",
            IssuingCardSpendingLimitCategories::ComputerSoftwareStores => "computer_software_stores",
            IssuingCardSpendingLimitCategories::ComputersPeripheralsAndSoftware => "computers_peripherals_and_software",
            IssuingCardSpendingLimitCategories::ConcreteWorkServices => "concrete_work_services",
            IssuingCardSpendingLimitCategories::ConstructionMaterials => "construction_materials",
            IssuingCardSpendingLimitCategories::ConsultingPublicRelations => "consulting_public_relations",
            IssuingCardSpendingLimitCategories::CorrespondenceSchools => "correspondence_schools",
            IssuingCardSpendingLimitCategories::CosmeticStores => "cosmetic_stores",
            IssuingCardSpendingLimitCategories::CounselingServices => "counseling_services",
            IssuingCardSpendingLimitCategories::CountryClubs => "country_clubs",
            IssuingCardSpendingLimitCategories::CourierServices => "courier_services",
            IssuingCardSpendingLimitCategories::CourtCosts => "court_costs",
            IssuingCardSpendingLimitCategories::CreditReportingAgencies => "credit_reporting_agencies",
            IssuingCardSpendingLimitCategories::CruiseLines => "cruise_lines",
            IssuingCardSpendingLimitCategories::DairyProductsStores => "dairy_products_stores",
            IssuingCardSpendingLimitCategories::DanceHallStudiosSchools => "dance_hall_studios_schools",
            IssuingCardSpendingLimitCategories::DatingEscortServices => "dating_escort_services",
            IssuingCardSpendingLimitCategories::DentistsOrthodontists => "dentists_orthodontists",
            IssuingCardSpendingLimitCategories::DepartmentStores => "department_stores",
            IssuingCardSpendingLimitCategories::DetectiveAgencies => "detective_agencies",
            IssuingCardSpendingLimitCategories::DigitalGoodsApplications => "digital_goods_applications",
            IssuingCardSpendingLimitCategories::DigitalGoodsGames => "digital_goods_games",
            IssuingCardSpendingLimitCategories::DigitalGoodsLargeVolume => "digital_goods_large_volume",
            IssuingCardSpendingLimitCategories::DigitalGoodsMedia => "digital_goods_media",
            IssuingCardSpendingLimitCategories::DirectMarketingCatalogMerchant => "direct_marketing_catalog_merchant",
            IssuingCardSpendingLimitCategories::DirectMarketingCombinationCatalogAndRetailMerchant => "direct_marketing_combination_catalog_and_retail_merchant",
            IssuingCardSpendingLimitCategories::DirectMarketingInboundTelemarketing => "direct_marketing_inbound_telemarketing",
            IssuingCardSpendingLimitCategories::DirectMarketingInsuranceServices => "direct_marketing_insurance_services",
            IssuingCardSpendingLimitCategories::DirectMarketingOther => "direct_marketing_other",
            IssuingCardSpendingLimitCategories::DirectMarketingOutboundTelemarketing => "direct_marketing_outbound_telemarketing",
            IssuingCardSpendingLimitCategories::DirectMarketingSubscription => "direct_marketing_subscription",
            IssuingCardSpendingLimitCategories::DirectMarketingTravel => "direct_marketing_travel",
            IssuingCardSpendingLimitCategories::DiscountStores => "discount_stores",
            IssuingCardSpendingLimitCategories::Doctors => "doctors",
            IssuingCardSpendingLimitCategories::DoorToDoorSales => "door_to_door_sales",
            IssuingCardSpendingLimitCategories::DraperyWindowCoveringAndUpholsteryStores => "drapery_window_covering_and_upholstery_stores",
            IssuingCardSpendingLimitCategories::DrinkingPlaces => "drinking_places",
            IssuingCardSpendingLimitCategories::DrugStoresAndPharmacies => "drug_stores_and_pharmacies",
            IssuingCardSpendingLimitCategories::DrugsDrugProprietariesAndDruggistSundries => "drugs_drug_proprietaries_and_druggist_sundries",
            IssuingCardSpendingLimitCategories::DryCleaners => "dry_cleaners",
            IssuingCardSpendingLimitCategories::DurableGoods => "durable_goods",
            IssuingCardSpendingLimitCategories::DutyFreeStores => "duty_free_stores",
            IssuingCardSpendingLimitCategories::EatingPlacesRestaurants => "eating_places_restaurants",
            IssuingCardSpendingLimitCategories::EducationalServices => "educational_services",
            IssuingCardSpendingLimitCategories::ElectricRazorStores => "electric_razor_stores",
            IssuingCardSpendingLimitCategories::ElectricalPartsAndEquipment => "electrical_parts_and_equipment",
            IssuingCardSpendingLimitCategories::ElectricalServices => "electrical_services",
            IssuingCardSpendingLimitCategories::ElectronicsRepairShops => "electronics_repair_shops",
            IssuingCardSpendingLimitCategories::ElectronicsStores => "electronics_stores",
            IssuingCardSpendingLimitCategories::ElementarySecondarySchools => "elementary_secondary_schools",
            IssuingCardSpendingLimitCategories::EmploymentTempAgencies => "employment_temp_agencies",
            IssuingCardSpendingLimitCategories::EquipmentRental => "equipment_rental",
            IssuingCardSpendingLimitCategories::ExterminatingServices => "exterminating_services",
            IssuingCardSpendingLimitCategories::FamilyClothingStores => "family_clothing_stores",
            IssuingCardSpendingLimitCategories::FastFoodRestaurants => "fast_food_restaurants",
            IssuingCardSpendingLimitCategories::FinancialInstitutions => "financial_institutions",
            IssuingCardSpendingLimitCategories::FinesGovernmentAdministrativeEntities => "fines_government_administrative_entities",
            IssuingCardSpendingLimitCategories::FireplaceFireplaceScreensAndAccessoriesStores => "fireplace_fireplace_screens_and_accessories_stores",
            IssuingCardSpendingLimitCategories::FloorCoveringStores => "floor_covering_stores",
            IssuingCardSpendingLimitCategories::Florists => "florists",
            IssuingCardSpendingLimitCategories::FloristsSuppliesNurseryStockAndFlowers => "florists_supplies_nursery_stock_and_flowers",
            IssuingCardSpendingLimitCategories::FreezerAndLockerMeatProvisioners => "freezer_and_locker_meat_provisioners",
            IssuingCardSpendingLimitCategories::FuelDealersNonAutomotive => "fuel_dealers_non_automotive",
            IssuingCardSpendingLimitCategories::FuneralServicesCrematories => "funeral_services_crematories",
            IssuingCardSpendingLimitCategories::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances => "furniture_home_furnishings_and_equipment_stores_except_appliances",
            IssuingCardSpendingLimitCategories::FurnitureRepairRefinishing => "furniture_repair_refinishing",
            IssuingCardSpendingLimitCategories::FurriersAndFurShops => "furriers_and_fur_shops",
            IssuingCardSpendingLimitCategories::GeneralServices => "general_services",
            IssuingCardSpendingLimitCategories::GiftCardNoveltyAndSouvenirShops => "gift_card_novelty_and_souvenir_shops",
            IssuingCardSpendingLimitCategories::GlassPaintAndWallpaperStores => "glass_paint_and_wallpaper_stores",
            IssuingCardSpendingLimitCategories::GlasswareCrystalStores => "glassware_crystal_stores",
            IssuingCardSpendingLimitCategories::GolfCoursesPublic => "golf_courses_public",
            IssuingCardSpendingLimitCategories::GovernmentServices => "government_services",
            IssuingCardSpendingLimitCategories::GroceryStoresSupermarkets => "grocery_stores_supermarkets",
            IssuingCardSpendingLimitCategories::HardwareEquipmentAndSupplies => "hardware_equipment_and_supplies",
            IssuingCardSpendingLimitCategories::HardwareStores => "hardware_stores",
            IssuingCardSpendingLimitCategories::HealthAndBeautySpas => "health_and_beauty_spas",
            IssuingCardSpendingLimitCategories::HearingAidsSalesAndSupplies => "hearing_aids_sales_and_supplies",
            IssuingCardSpendingLimitCategories::HeatingPlumbingAC => "heating_plumbing_a_c",
            IssuingCardSpendingLimitCategories::HobbyToyAndGameShops => "hobby_toy_and_game_shops",
            IssuingCardSpendingLimitCategories::HomeSupplyWarehouseStores => "home_supply_warehouse_stores",
            IssuingCardSpendingLimitCategories::Hospitals => "hospitals",
            IssuingCardSpendingLimitCategories::HotelsMotelsAndResorts => "hotels_motels_and_resorts",
            IssuingCardSpendingLimitCategories::HouseholdApplianceStores => "household_appliance_stores",
            IssuingCardSpendingLimitCategories::IndustrialSupplies => "industrial_supplies",
            IssuingCardSpendingLimitCategories::InformationRetrievalServices => "information_retrieval_services",
            IssuingCardSpendingLimitCategories::InsuranceDefault => "insurance_default",
            IssuingCardSpendingLimitCategories::InsuranceUnderwritingPremiums => "insurance_underwriting_premiums",
            IssuingCardSpendingLimitCategories::IntraCompanyPurchases => "intra_company_purchases",
            IssuingCardSpendingLimitCategories::JewelryStoresWatchesClocksAndSilverwareStores => "jewelry_stores_watches_clocks_and_silverware_stores",
            IssuingCardSpendingLimitCategories::LandscapingServices => "landscaping_services",
            IssuingCardSpendingLimitCategories::Laundries => "laundries",
            IssuingCardSpendingLimitCategories::LaundryCleaningServices => "laundry_cleaning_services",
            IssuingCardSpendingLimitCategories::LegalServicesAttorneys => "legal_services_attorneys",
            IssuingCardSpendingLimitCategories::LuggageAndLeatherGoodsStores => "luggage_and_leather_goods_stores",
            IssuingCardSpendingLimitCategories::LumberBuildingMaterialsStores => "lumber_building_materials_stores",
            IssuingCardSpendingLimitCategories::ManualCashDisburse => "manual_cash_disburse",
            IssuingCardSpendingLimitCategories::MarinasServiceAndSupplies => "marinas_service_and_supplies",
            IssuingCardSpendingLimitCategories::MasonryStoneworkAndPlaster => "masonry_stonework_and_plaster",
            IssuingCardSpendingLimitCategories::MassageParlors => "massage_parlors",
            IssuingCardSpendingLimitCategories::MedicalAndDentalLabs => "medical_and_dental_labs",
            IssuingCardSpendingLimitCategories::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies => "medical_dental_ophthalmic_and_hospital_equipment_and_supplies",
            IssuingCardSpendingLimitCategories::MedicalServices => "medical_services",
            IssuingCardSpendingLimitCategories::MembershipOrganizations => "membership_organizations",
            IssuingCardSpendingLimitCategories::MensAndBoysClothingAndAccessoriesStores => "mens_and_boys_clothing_and_accessories_stores",
            IssuingCardSpendingLimitCategories::MensWomensClothingStores => "mens_womens_clothing_stores",
            IssuingCardSpendingLimitCategories::MetalServiceCenters => "metal_service_centers",
            IssuingCardSpendingLimitCategories::Miscellaneous => "miscellaneous",
            IssuingCardSpendingLimitCategories::MiscellaneousApparelAndAccessoryShops => "miscellaneous_apparel_and_accessory_shops",
            IssuingCardSpendingLimitCategories::MiscellaneousAutoDealers => "miscellaneous_auto_dealers",
            IssuingCardSpendingLimitCategories::MiscellaneousBusinessServices => "miscellaneous_business_services",
            IssuingCardSpendingLimitCategories::MiscellaneousFoodStores => "miscellaneous_food_stores",
            IssuingCardSpendingLimitCategories::MiscellaneousGeneralMerchandise => "miscellaneous_general_merchandise",
            IssuingCardSpendingLimitCategories::MiscellaneousGeneralServices => "miscellaneous_general_services",
            IssuingCardSpendingLimitCategories::MiscellaneousHomeFurnishingSpecialtyStores => "miscellaneous_home_furnishing_specialty_stores",
            IssuingCardSpendingLimitCategories::MiscellaneousPublishingAndPrinting => "miscellaneous_publishing_and_printing",
            IssuingCardSpendingLimitCategories::MiscellaneousRecreationServices => "miscellaneous_recreation_services",
            IssuingCardSpendingLimitCategories::MiscellaneousRepairShops => "miscellaneous_repair_shops",
            IssuingCardSpendingLimitCategories::MiscellaneousSpecialtyRetail => "miscellaneous_specialty_retail",
            IssuingCardSpendingLimitCategories::MobileHomeDealers => "mobile_home_dealers",
            IssuingCardSpendingLimitCategories::MotionPictureTheaters => "motion_picture_theaters",
            IssuingCardSpendingLimitCategories::MotorFreightCarriersAndTrucking => "motor_freight_carriers_and_trucking",
            IssuingCardSpendingLimitCategories::MotorHomesDealers => "motor_homes_dealers",
            IssuingCardSpendingLimitCategories::MotorVehicleSuppliesAndNewParts => "motor_vehicle_supplies_and_new_parts",
            IssuingCardSpendingLimitCategories::MotorcycleShopsAndDealers => "motorcycle_shops_and_dealers",
            IssuingCardSpendingLimitCategories::MotorcycleShopsDealers => "motorcycle_shops_dealers",
            IssuingCardSpendingLimitCategories::MusicStoresMusicalInstrumentsPianosAndSheetMusic => "music_stores_musical_instruments_pianos_and_sheet_music",
            IssuingCardSpendingLimitCategories::NewsDealersAndNewsstands => "news_dealers_and_newsstands",
            IssuingCardSpendingLimitCategories::NonFiMoneyOrders => "non_fi_money_orders",
            IssuingCardSpendingLimitCategories::NonFiStoredValueCardPurchaseLoad => "non_fi_stored_value_card_purchase_load",
            IssuingCardSpendingLimitCategories::NondurableGoods => "nondurable_goods",
            IssuingCardSpendingLimitCategories::NurseriesLawnAndGardenSupplyStores => "nurseries_lawn_and_garden_supply_stores",
            IssuingCardSpendingLimitCategories::NursingPersonalCare => "nursing_personal_care",
            IssuingCardSpendingLimitCategories::OfficeAndCommercialFurniture => "office_and_commercial_furniture",
            IssuingCardSpendingLimitCategories::OpticiansEyeglasses => "opticians_eyeglasses",
            IssuingCardSpendingLimitCategories::OptometristsOphthalmologist => "optometrists_ophthalmologist",
            IssuingCardSpendingLimitCategories::OrthopedicGoodsProstheticDevices => "orthopedic_goods_prosthetic_devices",
            IssuingCardSpendingLimitCategories::Osteopaths => "osteopaths",
            IssuingCardSpendingLimitCategories::PackageStoresBeerWineAndLiquor => "package_stores_beer_wine_and_liquor",
            IssuingCardSpendingLimitCategories::PaintsVarnishesAndSupplies => "paints_varnishes_and_supplies",
            IssuingCardSpendingLimitCategories::ParkingLotsGarages => "parking_lots_garages",
            IssuingCardSpendingLimitCategories::PassengerRailways => "passenger_railways",
            IssuingCardSpendingLimitCategories::PawnShops => "pawn_shops",
            IssuingCardSpendingLimitCategories::PetShopsPetFoodAndSupplies => "pet_shops_pet_food_and_supplies",
            IssuingCardSpendingLimitCategories::PetroleumAndPetroleumProducts => "petroleum_and_petroleum_products",
            IssuingCardSpendingLimitCategories::PhotoDeveloping => "photo_developing",
            IssuingCardSpendingLimitCategories::PhotographicPhotocopyMicrofilmEquipmentAndSupplies => "photographic_photocopy_microfilm_equipment_and_supplies",
            IssuingCardSpendingLimitCategories::PhotographicStudios => "photographic_studios",
            IssuingCardSpendingLimitCategories::PictureVideoProduction => "picture_video_production",
            IssuingCardSpendingLimitCategories::PieceGoodsNotionsAndOtherDryGoods => "piece_goods_notions_and_other_dry_goods",
            IssuingCardSpendingLimitCategories::PlumbingHeatingEquipmentAndSupplies => "plumbing_heating_equipment_and_supplies",
            IssuingCardSpendingLimitCategories::PoliticalOrganizations => "political_organizations",
            IssuingCardSpendingLimitCategories::PostalServicesGovernmentOnly => "postal_services_government_only",
            IssuingCardSpendingLimitCategories::PreciousStonesAndMetalsWatchesAndJewelry => "precious_stones_and_metals_watches_and_jewelry",
            IssuingCardSpendingLimitCategories::ProfessionalServices => "professional_services",
            IssuingCardSpendingLimitCategories::PublicWarehousingAndStorage => "public_warehousing_and_storage",
            IssuingCardSpendingLimitCategories::QuickCopyReproAndBlueprint => "quick_copy_repro_and_blueprint",
            IssuingCardSpendingLimitCategories::Railroads => "railroads",
            IssuingCardSpendingLimitCategories::RealEstateAgentsAndManagersRentals => "real_estate_agents_and_managers_rentals",
            IssuingCardSpendingLimitCategories::RecordStores => "record_stores",
            IssuingCardSpendingLimitCategories::RecreationalVehicleRentals => "recreational_vehicle_rentals",
            IssuingCardSpendingLimitCategories::ReligiousGoodsStores => "religious_goods_stores",
            IssuingCardSpendingLimitCategories::ReligiousOrganizations => "religious_organizations",
            IssuingCardSpendingLimitCategories::RoofingSidingSheetMetal => "roofing_siding_sheet_metal",
            IssuingCardSpendingLimitCategories::SecretarialSupportServices => "secretarial_support_services",
            IssuingCardSpendingLimitCategories::SecurityBrokersDealers => "security_brokers_dealers",
            IssuingCardSpendingLimitCategories::ServiceStations => "service_stations",
            IssuingCardSpendingLimitCategories::SewingNeedleworkFabricAndPieceGoodsStores => "sewing_needlework_fabric_and_piece_goods_stores",
            IssuingCardSpendingLimitCategories::ShoeRepairHatCleaning => "shoe_repair_hat_cleaning",
            IssuingCardSpendingLimitCategories::ShoeStores => "shoe_stores",
            IssuingCardSpendingLimitCategories::SmallApplianceRepair => "small_appliance_repair",
            IssuingCardSpendingLimitCategories::SnowmobileDealers => "snowmobile_dealers",
            IssuingCardSpendingLimitCategories::SpecialTradeServices => "special_trade_services",
            IssuingCardSpendingLimitCategories::SpecialtyCleaning => "specialty_cleaning",
            IssuingCardSpendingLimitCategories::SportingGoodsStores => "sporting_goods_stores",
            IssuingCardSpendingLimitCategories::SportingRecreationCamps => "sporting_recreation_camps",
            IssuingCardSpendingLimitCategories::SportsAndRidingApparelStores => "sports_and_riding_apparel_stores",
            IssuingCardSpendingLimitCategories::SportsClubsFields => "sports_clubs_fields",
            IssuingCardSpendingLimitCategories::StampAndCoinStores => "stamp_and_coin_stores",
            IssuingCardSpendingLimitCategories::StationaryOfficeSuppliesPrintingAndWritingPaper => "stationary_office_supplies_printing_and_writing_paper",
            IssuingCardSpendingLimitCategories::StationeryStoresOfficeAndSchoolSupplyStores => "stationery_stores_office_and_school_supply_stores",
            IssuingCardSpendingLimitCategories::SwimmingPoolsSales => "swimming_pools_sales",
            IssuingCardSpendingLimitCategories::TUiTravelGermany => "t_ui_travel_germany",
            IssuingCardSpendingLimitCategories::TailorsAlterations => "tailors_alterations",
            IssuingCardSpendingLimitCategories::TaxPaymentsGovernmentAgencies => "tax_payments_government_agencies",
            IssuingCardSpendingLimitCategories::TaxPreparationServices => "tax_preparation_services",
            IssuingCardSpendingLimitCategories::TaxicabsLimousines => "taxicabs_limousines",
            IssuingCardSpendingLimitCategories::TelecommunicationEquipmentAndTelephoneSales => "telecommunication_equipment_and_telephone_sales",
            IssuingCardSpendingLimitCategories::TelecommunicationServices => "telecommunication_services",
            IssuingCardSpendingLimitCategories::TelegraphServices => "telegraph_services",
            IssuingCardSpendingLimitCategories::TentAndAwningShops => "tent_and_awning_shops",
            IssuingCardSpendingLimitCategories::TestingLaboratories => "testing_laboratories",
            IssuingCardSpendingLimitCategories::TheatricalTicketAgencies => "theatrical_ticket_agencies",
            IssuingCardSpendingLimitCategories::Timeshares => "timeshares",
            IssuingCardSpendingLimitCategories::TireRetreadingAndRepair => "tire_retreading_and_repair",
            IssuingCardSpendingLimitCategories::TollsBridgeFees => "tolls_bridge_fees",
            IssuingCardSpendingLimitCategories::TouristAttractionsAndExhibits => "tourist_attractions_and_exhibits",
            IssuingCardSpendingLimitCategories::TowingServices => "towing_services",
            IssuingCardSpendingLimitCategories::TrailerParksCampgrounds => "trailer_parks_campgrounds",
            IssuingCardSpendingLimitCategories::TransportationServices => "transportation_services",
            IssuingCardSpendingLimitCategories::TravelAgenciesTourOperators => "travel_agencies_tour_operators",
            IssuingCardSpendingLimitCategories::TruckStopIteration => "truck_stop_iteration",
            IssuingCardSpendingLimitCategories::TruckUtilityTrailerRentals => "truck_utility_trailer_rentals",
            IssuingCardSpendingLimitCategories::TypesettingPlateMakingAndRelatedServices => "typesetting_plate_making_and_related_services",
            IssuingCardSpendingLimitCategories::TypewriterStores => "typewriter_stores",
            IssuingCardSpendingLimitCategories::USFederalGovernmentAgenciesOrDepartments => "u_s_federal_government_agencies_or_departments",
            IssuingCardSpendingLimitCategories::UniformsCommercialClothing => "uniforms_commercial_clothing",
            IssuingCardSpendingLimitCategories::UsedMerchandiseAndSecondhandStores => "used_merchandise_and_secondhand_stores",
            IssuingCardSpendingLimitCategories::Utilities => "utilities",
            IssuingCardSpendingLimitCategories::VarietyStores => "variety_stores",
            IssuingCardSpendingLimitCategories::VeterinaryServices => "veterinary_services",
            IssuingCardSpendingLimitCategories::VideoAmusementGameSupplies => "video_amusement_game_supplies",
            IssuingCardSpendingLimitCategories::VideoGameArcades => "video_game_arcades",
            IssuingCardSpendingLimitCategories::VideoTapeRentalStores => "video_tape_rental_stores",
            IssuingCardSpendingLimitCategories::VocationalTradeSchools => "vocational_trade_schools",
            IssuingCardSpendingLimitCategories::WatchJewelryRepair => "watch_jewelry_repair",
            IssuingCardSpendingLimitCategories::WeldingRepair => "welding_repair",
            IssuingCardSpendingLimitCategories::WholesaleClubs => "wholesale_clubs",
            IssuingCardSpendingLimitCategories::WigAndToupeeStores => "wig_and_toupee_stores",
            IssuingCardSpendingLimitCategories::WiresMoneyOrders => "wires_money_orders",
            IssuingCardSpendingLimitCategories::WomensAccessoryAndSpecialtyShops => "womens_accessory_and_specialty_shops",
            IssuingCardSpendingLimitCategories::WomensReadyToWearStores => "womens_ready_to_wear_stores",
            IssuingCardSpendingLimitCategories::WreckingAndSalvageYards => "wrecking_and_salvage_yards",
        }
    }
}

impl AsRef<str> for IssuingCardSpendingLimitCategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardSpendingLimitCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCardSpendingLimit`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardSpendingLimitInterval {
    AllTime,
    Daily,
    Monthly,
    PerAuthorization,
    Weekly,
    Yearly,
}

impl IssuingCardSpendingLimitInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardSpendingLimitInterval::AllTime => "all_time",
            IssuingCardSpendingLimitInterval::Daily => "daily",
            IssuingCardSpendingLimitInterval::Monthly => "monthly",
            IssuingCardSpendingLimitInterval::PerAuthorization => "per_authorization",
            IssuingCardSpendingLimitInterval::Weekly => "weekly",
            IssuingCardSpendingLimitInterval::Yearly => "yearly",
        }
    }
}

impl AsRef<str> for IssuingCardSpendingLimitInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardSpendingLimitInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCard`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardStatus {
    Active,
    Canceled,
    Inactive,
}

impl IssuingCardStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardStatus::Active => "active",
            IssuingCardStatus::Canceled => "canceled",
            IssuingCardStatus::Inactive => "inactive",
        }
    }
}

impl AsRef<str> for IssuingCardStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

//automatically added back in service of IssuingCard with hash4501721549196130287
impl Object for IssuingCard {
    type Id = IssuingCardId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.card"
    }
}
