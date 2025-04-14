import { useEffect } from "react";
import { Link, useNavigate, useParams } from "react-router";
import { Breadcrumbs, Divider, Title } from "@mantine/core";
import { DefaultLayout, UserForm } from "@components";
import { ApiResponseError, User } from "@entity";
import { useUserApi, useUserForm, UserFormValuesType } from "@hooks";
import { ErrorMessage, useErrorMessage } from "@shared/ui";

export function UserEditPage() {
  const navigate = useNavigate();
  const { id } = useParams();
  const {
    errorMessageConfig,
    visibleErrorMessage,
    showErrorMessage,
    hideErrorMessage,
  } = useErrorMessage();
  const { getUserApi, updateUserApi } = useUserApi();
  const { form, invalid, updateFormValues } = useUserForm();

  const handleSave = form.onSubmit((formValues: UserFormValuesType) => {
    const payload: User = { ...formValues };

    hideErrorMessage();
    updateUserApi(payload)
      .then(() => navigate(`/users/${id}`))
      .catch((err: ApiResponseError) => {
        showErrorMessage({
          title: "システムエラー",
          message: err.message,
        });
      });
  });

  useEffect(() => {
    if (id !== undefined && id !== "") {
      getUserApi(Number(id))
        .then((res) => updateFormValues(res.data.user))
        .catch((err: ApiResponseError) => {
          if (err.status === 404) {
            navigate("/notfound");
          }
          showErrorMessage({
            title: "システムエラー",
            message: err.message,
          });
        });
    }
  }, [id, getUserApi, navigate]);

  useEffect(() => {
    if (invalid) {
      showErrorMessage({
        title: "入力エラー",
        message: "入力内容に誤りがあります",
      });
    }
  }, [invalid, showErrorMessage]);

  return (
    <DefaultLayout>
      <Breadcrumbs separator=">">
        <Link to={`/users/${id}`}>ユーザー情報</Link>
        <span>ユーザー情報変更</span>
      </Breadcrumbs>
      <Title order={2} mt="md">
        ユーザー情報変更
      </Title>
      <Divider my="md" />
      <ErrorMessage config={errorMessageConfig} visible={visibleErrorMessage} />
      <UserForm form={form} onSave={handleSave} />
    </DefaultLayout>
  );
}
