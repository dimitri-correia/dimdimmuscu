// pub async fn mw_ctx_resolver(
//     State(db): State<DbInfos>,
//     cookies: Cookies,
//     mut req: Request<Body>,
//     next: Next,
// ) -> Response {
//     debug!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");
//
//     let ctx_ext_result = ctx_resolve(db, &cookies).await;
//
//     if ctx_ext_result.is_err() && !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie)) {
//         cookies.remove(Cookie::from(AUTH_TOKEN))
//     }
//
//     req.extensions_mut().insert(ctx_ext_result);
//
//     next.run(req).await
// }
//
// async fn ctx_resolve(db_infos: DbInfos, cookies: &Cookies) -> Result {
//     // -- Get Token String
//     let token = cookies
//         .get(AUTH_TOKEN)
//         .map(|c| c.value().to_string())
//         .ok_or(CtxExtError::TokenNotInCookie)?;
//
//     // -- Parse Token
//     let token: Token = token.parse().map_err(|_| CtxExtError::TokenWrongFormat)?;
//
//     // -- Get UserForAuth
//     let user: UserForLogin = UserForLogin::find_by_name(&db_infos, &token.ident)
//         .await
//         .map_err(|ex| CtxExtError::ModelAccessError(ex.to_string()))?
//         .ok_or(CtxExtError::UserNotFound)?;
//
//     // -- Validate Token
//     validate_web_token(&token, user.token_salt).map_err(|_| CtxExtError::FailValidate)?;
//
//     // -- Update Token
//     set_token_cookie(cookies, &user.username, user.token_salt)
//         .map_err(|_| CtxExtError::CannotSetTokenCookie)?;
//
//     // -- Create CtxExtResult
//     Ctx::new(user.id)
//         .map(CtxW)
//         .map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))
// }
