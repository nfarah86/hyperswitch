/// Create Payment Methods
///
/// Creates and stores a payment method against a customer.
/// This endpoint supports a variety of payment methods including cards, wallets, and bank transfers.
/// 
/// In case of cards, this API should only be used by PCI-compliant merchants.
/// For non-PCI compliant use cases, use the `/payment-method-session` flow instead.
#[utoipa::path(
    post,
    path = "/payment_methods",
    request_body (
        content = PaymentMethodCreate,
        examples  (( "Save a credit card" =(
        summary = "Store a Visa credit card for a customer",
        value =json!( {
            "payment_method": "card",
            "payment_method_type": "credit",
            "payment_method_issuer": "Visa",
            "card": {
            "card_number": "4242424242424242",
            "card_exp_month": "11",
            "card_exp_year": "25",
            "card_holder_name": "John Doe"
            },
            "customer_id": "{{customer_id}}"
        })
        )))
    ),
    responses(
        (status = 200, description = "Payment method successfully created", body = PaymentMethodResponse),
        (status = 400, description = "Invalid input; check request formatting and required feels")

    ),
    tag = "Payment Methods",
    operation_id = "Create a Payment Method",
    security(("api_key" = []))
)]
#[cfg(feature = "v1")]
pub async fn create_payment_method_api() {}

/// List payment methods for a Merchant
///
/// Retrieves the available payment methods configured for a given merchant account.
/// This includes cards, wallets, bank transfers, and any other enabled methods based on geography, currency, and business configuration.
///
/// This endpoint requires both `api_key` and `publishable_key` for authorization.
///
/// Use this to dynamically render available payment options during checkout, based on the payment amount, country, and currency.

#[utoipa::path(
    get,
    path = "/account/payment_methods",
    params (
        ("account_id" = String, Path, description = "The unique identifier for the merchant account"),
        ("accepted_country" = Vec<String>, Query, description = "Two-letter ISO currency code(e.g., 'US', 'IN', to filter methods available in specified regions)"),
        ("accepted_currency" = Vec<Currency>, Path, description = "Three-letter ISO currency code(e.g., 'USD', 'INR') to filter methods available in specified regions"),
        ("minimum_amount" = i64, Query, description = "Minimum transaction amount (in minor units like cents or paise) supported by the method"),
        ("maximum_amount" = i64, Query, description = "Maximum transaction amount (in minor units) supported by the method."),
        ("recurring_payment_enabled" = bool, Query, description = "Filter methods that support recurring transactions."),
        ("installment_payment_enabled" = bool, Query, description = "Filter methods that support installment payments."),
    ),
    responses(
        (status = 200, description = "List of payment methods returned successfully.", body = PaymentMethodListResponse),
        (status = 400, description =  "Invalid query parameters."),
        (status = 404, description = "No applicable payment methods found.")
    ),
    tag = "Payment Methods",
    operation_id = "List all Payment Methods for a Merchant",
    security(("api_key" = []), ("publishable_key" = []))
)]
pub async fn list_payment_method_api() {}

/// List payment methods for a Customer
///
/// Returns applicable payment methods for the given customer ID. This includes saved payment methods
/// (such as stored cards or wallets) as well as dynamically available options based on country, currency,
/// and amount range.
///
/// Use this to render available payment options to a returning customer at checkout.
///
/// Requires an `api_key` for server-side access.
#[utoipa::path(
    get,
    path = "/customers/{customer_id}/payment_methods",
    params (
        ("customer_id" = String, Path, description = "Unique identifier for the customer."),
        ("accepted_country" = Vec<String>, Query, description = "Two-letter ISO country codes (e.g. 'US', 'IN') to filter methods available in specified regions."),
        ("accepted_currency" = Vec<Currency>, Path, description = "The three-letter ISO currency code (e.g. 'USD', 'INR') to filter supported currencies."),
        ("minimum_amount" = i64, Query, description = "Minimum transaction amount (in minor units like cents or paise)."),
        ("maximum_amount" = i64, Query, description = "Maximum transaction amount (in minor units)."),
        ("recurring_payment_enabled" = bool, Query, description = "Filter methods that support recurring transactions."),
        ("installment_payment_enabled" = bool, Query, description = "Filter methods that support installment payments."),
    ),
    responses(
        (status = 200, description = "Successfully returned applicable payment methods for the customer.", body = CustomerPaymentMethodsListResponse),
        (status = 400, description = "One or more query parameters are invalid."),
        (status = 404, description = "No applicable payment methods found for the given customer.")
    ),
    tag = "Payment Methods",
    operation_id = "List all Payment Methods for a Customer",
    security(("api_key" = []))
)]
#[cfg(feature = "v1")]
pub async fn list_customer_payment_method_api() {}

/// List customer payment methods using a client secret
///
/// Returns applicable payment methods (including saved and eligible ones) for a given customer,
/// using the `client_secret` for client-side authenticated contexts (e.g. browser or mobile apps).
///
/// This is typically used in hosted checkout or embedded payment flows where the server has issued a
/// `client_secret` and the frontend retrieves payment methods directly.
///
/// Requires `publishable_key` (not `api_key`), as this is intended for client-side use.
#[utoipa::path(
    get,
    path = "/customers/payment_methods",
         ("client-secret" = String, Path, description = "Client-side secret used to authenticate and fetch payment methods for a specific payment."),
        ("customer_id" = String, Path, description = "Unique identifier for the customer."),
        ("accepted_country" = Vec<String>, Query, description = "Two-letter ISO country codes (e.g. 'US', 'IN') to filter methods by region."),
        ("accepted_currency" = Vec<Currency>, Path, description = "Three-letter ISO currency codes (e.g. 'USD', 'INR') to filter methods by currency."),
        ("minimum_amount" = i64, Query, description = "Minimum transaction amount (in minor units like cents or paise)."),
        ("maximum_amount" = i64, Query, description = "Maximum transaction amount (in minor units)."),
        ("recurring_payment_enabled" = bool, Query, description = "Return only methods eligible for recurring billing."),
        ("installment_payment_enabled" = bool, Query, description = "Return only methods that support installment payments."),
    ),
    responses(
        (status = 200, description = "Successfully retrieved payment methods available for the client-secret and customer.", body = CustomerPaymentMethodsListResponse),
        (status = 400, description = "Invalid query parameters or missing client-secret."),
        (status = 404, description = "No applicable payment methods found for the given customer and client-secret.")
    ),
    tag = "Payment Methods",
    operation_id = "List Customer Payment Methods via Client Secret",
    security(("publishable_key" = []))
)]
pub async fn list_customer_payment_method_api_client() {}

/// Retrieve Payment Method
///
/// Retrieves a payment method of a customer.
#[utoipa::path(
    get,
    path = "/payment_methods/{method_id}",
    params (
        ("method_id" = String, Path, description = "The unique identifier for the Payment Method"),
    ),
    responses(
        (status = 200, description = "Payment Method is successfully retrieved", body = PaymentMethodResponse),
        (status = 404, description = "Payment Method does not exist in records")
    ),
    tag = "Payment Methods",
    operation_id = "Retrieve a Payment method",
    security(("api_key" = []))
)]
#[cfg(feature = "v1")]
pub async fn payment_method_retrieve_api() {}

/// Update a Payment Method
/// Updates an existing saved payment method for a customer. This is commonly used to update card details
/// (such as expiration date or cardholder name) in order to maintain continuity for recurring or subscription payments.
///
/// For example, if a customer’s card is expiring, this endpoint can be used to replace it with the updated details
/// without requiring re-authentication.
///
/// Requires both `api_key` and `publishable_key` for secure access.
#[utoipa::path(
    post,
    path = "/payment_methods/{method_id}/update",
    params (
        ("method_id" = String, Path, description = "The unique identifier for the Payment Method"),
    ),
    request_body = PaymentMethodUpdate,
    responses(
        (status = 200, description = "The payment method was successfully updated.", body = PaymentMethodResponse),
        (status = 404, description = "Payment method not found.")
    ),
    tag = "Payment Methods",
    operation_id = "Update a Payment method",
    security(("api_key" = []), ("publishable_key" = []))
)]
#[cfg(feature = "v1")]
pub async fn payment_method_update_api() {}

/// Delete a Payment Method
///
/// Permanently deletes a saved payment method associated with a customer account.
/// This is typically used when a customer wants to remove a stored card, bank account, or wallet from their profile.
///
/// Deleted payment methods can no longer be used for future transactions or recurring billing.
///
/// Requires `api_key` for authenticated access.
#[utoipa::path(
    delete,
    path = "/payment_methods/{method_id}",
    params (
        ("method_id" = String, Path, description = "Unique identifier for the Payment Method"),
    ),
    responses(
        (status = 200, description = "The payment method was successfully deleted.", body = PaymentMethodDeleteResponse),
        (status = 404, description = "No payment method found with the provided identifier.")
    ),
    tag = "Payment Methods",
    operation_id = "Delete a Payment method",
    security(("api_key" = []))
)]
#[cfg(feature = "v1")]
pub async fn payment_method_delete_api() {}

/// Set Default Payment Method for a Customer
///
/// Marks the specified payment method as the default for a given customer. Once set, this payment method
/// will be prioritized for future one-click, recurring, or subscription-based transactions unless overridden.
///
/// This is commonly used in settings or checkout flows where a customer selects their preferred saved method.
///
/// Requires `ephemeral_key` issued for the specific customer session.
#[utoipa::path(
    post,
    path = "/{customer_id}/payment_methods/{payment_method_id}/default",
    params (
        ("customer_id" = String,Path, description ="Unique identifier for the Customer"),
        ("payment_method_id" = String,Path, description = "Unique identifier for the Payment Method"),
    ),
    responses(
        (status = 200, description = "The payment method has been successfully set as the customer's default.", body =CustomerDefaultPaymentMethodResponse ),
        (status = 400, description = "The payment method is already the customer's default."),
        (status = 404, description = "The specified payment method was not found for the customer.")
    ),
    tag = "Payment Methods",
    operation_id = "Set the Payment Method as Default",
    security(("ephemeral_key" = []))
)]
pub async fn default_payment_method_set_api() {}

/// Payment Method - Create Intent
///
/// Creates a payment method for customer with billing information and other metadata.
#[utoipa::path(
    post,
    path = "/v2/payment-methods/create-intent",
    request_body(
    content = PaymentMethodIntentCreate,
    // TODO: Add examples
    ),
    responses(
        (status = 200, description = "Payment Method Intent Created", body = PaymentMethodResponse),
        (status = 400, description = "Invalid Data"),
    ),
    tag = "Payment Methods",
    operation_id = "Create Payment Method Intent",
    security(("api_key" = []))
)]
#[cfg(feature = "v2")]
pub async fn create_payment_method_intent_api() {}

/// Payment Method - Confirm Intent
///
/// Update a payment method with customer's payment method related information.
#[utoipa::path(
    post,
    path = "/v2/payment-methods/{id}/confirm-intent",
    request_body(
    content = PaymentMethodIntentConfirm,
    // TODO: Add examples
    ),
    responses(
        (status = 200, description = "Payment Method Intent Confirmed", body = PaymentMethodResponse),
        (status = 400, description = "Invalid Data"),
    ),
    tag = "Payment Methods",
    operation_id = "Confirm Payment Method Intent",
    security(("api_key" = []))
)]
#[cfg(feature = "v2")]
pub async fn confirm_payment_method_intent_api() {}

/// Payment Method - Create
///
/// Creates and stores a payment method against a customer. In case of cards, this API should be used only by PCI compliant merchants.
#[utoipa::path(
    post,
    path = "/v2/payment-methods",
    request_body(
    content = PaymentMethodCreate,
    // TODO: Add examples
    ),
    responses(
        (status = 200, description = "Payment Method Created", body = PaymentMethodResponse),
        (status = 400, description = "Invalid Data"),
    ),
    tag = "Payment Methods",
    operation_id = "Create Payment Method",
    security(("api_key" = []))
)]
#[cfg(feature = "v2")]
pub async fn create_payment_method_api() {}

/// Payment Method - Retrieve
///
/// Retrieves a payment method of a customer.
#[utoipa::path(
    get,
    path = "/v2/payment-methods/{id}",
    params (
        ("id" = String, Path, description = "The unique identifier for the Payment Method"),
    ),
    responses(
        (status = 200, description = "Payment Method Retrieved", body = PaymentMethodResponse),
        (status = 404, description = "Payment Method Not Found"),
    ),
    tag = "Payment Methods",
    operation_id = "Retrieve Payment Method",
    security(("api_key" = []))
)]
#[cfg(feature = "v2")]
pub async fn payment_method_retrieve_api() {}

/// Payment Method - Update
///
/// Update an existing payment method of a customer.
#[utoipa::path(
    patch,
    path = "/v2/payment-methods/{id}/update-saved-payment-method",
    request_body(
    content = PaymentMethodUpdate,
    // TODO: Add examples
    ),
    responses(
        (status = 200, description = "Payment Method Update", body = PaymentMethodResponse),
        (status = 400, description = "Invalid Data"),
    ),
    tag = "Payment Methods",
    operation_id = "Update Payment Method",
    security(("api_key" = []))
)]
#[cfg(feature = "v2")]
pub async fn payment_method_update_api() {}

/// Payment Method - Delete
///
/// Deletes a payment method of a customer.
#[utoipa::path(
    delete,
    path = "/v2/payment-methods/{id}",
    params (
        ("id" = String, Path, description = "The unique identifier for the Payment Method"),
    ),
    responses(
        (status = 200, description = "Payment Method Retrieved", body = PaymentMethodDeleteResponse),
        (status = 404, description = "Payment Method Not Found"),
    ),
    tag = "Payment Methods",
    operation_id = "Delete Payment Method",
    security(("api_key" = []))
)]
#[cfg(feature = "v2")]
pub async fn payment_method_delete_api() {}

/// Payment Method - List Customer Saved Payment Methods
///
/// List the payment methods saved for a customer
#[utoipa::path(
    get,
    path = "/v2/customers/{id}/saved-payment-methods",
    params (
        ("id" = String, Path, description = "The unique identifier for the customer"),
    ),
    responses(
        (status = 200, description = "Payment Methods Retrieved", body = CustomerPaymentMethodsListResponse),
        (status = 404, description = "Customer Not Found"),
    ),
    tag = "Payment Methods",
    operation_id = "List Customer Saved Payment Methods",
    security(("api_key" = []))
)]
#[cfg(feature = "v2")]
pub async fn list_customer_payment_method_api() {}

/// Payment Method Session - Create
///
/// Create a payment method session for a customer
/// This is used to list the saved payment methods for the customer
/// The customer can also add a new payment method using this session
#[cfg(feature = "v2")]
#[utoipa::path(
    post,
    path = "/v2/payment-method-session",
    request_body(
    content = PaymentMethodSessionRequest,
        examples  (( "Create a payment method session with customer_id" = (
        value =json!( {
            "customer_id": "12345_cus_abcdefghijklmnopqrstuvwxyz"
        })
        )))
    ),
    responses(
        (status = 200, description = "Create the payment method session", body = PaymentMethodSessionResponse),
        (status = 400, description = "The request is invalid")
    ),
    tag = "Payment Method Session",
    operation_id = "Create a payment method session",
    security(("api_key" = []))
)]
pub fn payment_method_session_create() {}

/// Payment Method Session - Retrieve
///
/// Retrieve the payment method session
#[cfg(feature = "v2")]
#[utoipa::path(
    get,
    path = "/v2/payment-method-session/{id}",
    params (
        ("id" = String, Path, description = "The unique identifier for the Payment Method Session"),
    ),
    responses(
        (status = 200, description = "The payment method session is retrieved successfully", body = PaymentMethodSessionResponse),
        (status = 404, description = "The request is invalid")
    ),
    tag = "Payment Method Session",
    operation_id = "Retrieve the payment method session",
    security(("ephemeral_key" = []))
)]
pub fn payment_method_session_retrieve() {}

/// Payment Method Session - List Payment Methods
///
/// List payment methods for the given payment method session.
/// This endpoint lists the enabled payment methods for the profile and the saved payment methods of the customer.
#[cfg(feature = "v2")]
#[utoipa::path(
    get,
    path = "/v2/payment-method-session/{id}/list-payment-methods",
    params (
        ("id" = String, Path, description = "The unique identifier for the Payment Method Session"),
    ),
    responses(
        (status = 200, description = "The payment method session is retrieved successfully", body = PaymentMethodListResponseForSession),
        (status = 404, description = "The request is invalid")
    ),
    tag = "Payment Method Session",
    operation_id = "List Payment methods for a Payment Method Session",
    security(("ephemeral_key" = []))
)]
pub fn payment_method_session_list_payment_methods() {}

/// Payment Method Session - Update a saved payment method
///
/// Update a saved payment method from the given payment method session.
#[cfg(feature = "v2")]
#[utoipa::path(
    put,
    path = "/v2/payment-method-session/{id}/update-saved-payment-method",
    params (
        ("id" = String, Path, description = "The unique identifier for the Payment Method Session"),
    ),
    request_body(
        content = PaymentMethodSessionUpdateSavedPaymentMethod,
            examples(( "Update the card holder name" = (
                value =json!( {
                    "payment_method_id": "12345_pm_0194b1ecabc172e28aeb71f70a4daba3",
                    "payment_method_data": {
                        "card": {
                            "card_holder_name": "Narayan Bhat"
                        }
                    }
                }
            )
        )))
    ),
    responses(
        (status = 200, description = "The payment method has been updated successfully", body = PaymentMethodResponse),
        (status = 404, description = "The request is invalid")
    ),
    tag = "Payment Method Session",
    operation_id = "Update a saved payment method",
    security(("ephemeral_key" = []))
)]
pub fn payment_method_session_update_saved_payment_method() {}

/// Payment Method Session - Delete a saved payment method
///
/// Delete a saved payment method from the given payment method session.
#[cfg(feature = "v2")]
#[utoipa::path(
    delete,
    path = "/v2/payment-method-session/{id}",
    params (
        ("id" = String, Path, description = "The unique identifier for the Payment Method Session"),
    ),
    request_body(
        content = PaymentMethodSessionDeleteSavedPaymentMethod,
            examples(( "Update the card holder name" = (
                value =json!( {
                    "payment_method_id": "12345_pm_0194b1ecabc172e28aeb71f70a4daba3",
                }
            )
        )))
    ),
    responses(
        (status = 200, description = "The payment method has been updated successfully", body = PaymentMethodDeleteResponse),
        (status = 404, description = "The request is invalid")
    ),
    tag = "Payment Method Session",
    operation_id = "Delete a saved payment method",
    security(("ephemeral_key" = []))
)]
pub fn payment_method_session_delete_saved_payment_method() {}

///Tokenize Card with Network - Create Payment Method
///
/// Securely tokenize raw card details using the card network (e.g. Visa, Mastercard)
/// and store the result as a reusable payment method for a customer.
///
/// This endpoint is typically used by PCI-compliant platforms during onboarding
/// or account linking flows to create a network token.
///
/// Use this if you want to store a card securely with enhanced network-level tokenization.
///
/// Requires `admin_api_key` and should only be used in secure, PCI-compliant server environments.
#[utoipa::path(
    post,
    path = "/payment_methods/tokenize-card",
    request_body = CardNetworkTokenizeRequest,
    responses(
        (status = 200, description = "Network token created and stored as a payment method", body = CardNetworkTokenizeResponse),
        (status = 404, description = "Customer not found"),
    ),
    tag = "Payment Methods",
    operation_id = "Create card network token",
    security(("admin_api_key" = []))
)]
pub async fn tokenize_card_api() {}

/// Tokenize Existing Card - Convert Saved Card to Network Token
///
/// Creates a card network token (e.g. via Visa or Mastercard) for an already saved payment method.
/// This is useful when upgrading a stored card to a network token for improved security and performance.
///
/// Use this endpoint when you already have a `payment_method_id` associated with a card
/// and want to enable tokenized payments.
///
/// Requires `admin_api_key` and should only be called from PCI-compliant, trusted server environments.
#[utoipa::path(
    post,
    path = "/payment_methods/{id}/tokenize-card",
    request_body = CardNetworkTokenizeRequest,
    params (
        ("id" = String, Path, description = "The unique identifier for the saved payment method"),
    ),
    responses(
        (status = 200, description = "Card network token created and stored", body = CardNetworkTokenizeResponse),
        (status = 404, description = "Payment Method or Customer not found"),
    ),
    tag = "Payment Methods",
    operation_id = "Create card network token using Payment Method ID",
    security(("admin_api_key" = []))
)]
pub async fn tokenize_card_using_pm_api() {}

/// Payment Method Session - Confirm a payment method session
///
/// **Confirms a payment method session object with the payment method data**
#[utoipa::path(
  post,
  path = "/v2/payment-method-session/{id}/confirm",
  params (("id" = String, Path, description = "The unique identifier for the Payment Method Session"),
      (
        "X-Profile-Id" = String, Header,
        description = "Profile ID associated to the payment intent",
        example = "pro_abcdefghijklmnop"
      )
    ),
  request_body(
      content = PaymentMethodSessionConfirmRequest,
      examples(
          (
              "Confirm the payment method session with card details" = (
                  value = json!({
                    "payment_method_type": "card",
                    "payment_method_subtype": "credit",
                    "payment_method_data": {
                      "card": {
                        "card_number": "4242424242424242",
                        "card_exp_month": "10",
                        "card_exp_year": "25",
                        "card_cvc": "123"
                      }
                    },
                  })
              )
          ),
      ),
  ),
  responses(
      (status = 200, description = "Payment Method created", body = PaymentMethodResponse),
      (status = 400, description = "Missing Mandatory fields")
  ),
  tag = "Payment Method Session",
  operation_id = "Confirm the payment method session",
  security(("publishable_key" = [])),
)]
#[cfg(feature = "v2")]
pub fn payment_method_session_confirm() {}
