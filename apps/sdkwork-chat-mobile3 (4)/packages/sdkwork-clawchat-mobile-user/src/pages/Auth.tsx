import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { Eye, EyeOff, MessageCircle } from "lucide-react";
import { cn, showToast } from "@sdkwork/clawchat-mobile-commons";
import { AuthService } from "../services/AuthService";
import { motion, AnimatePresence } from "motion/react";

type AuthMode = "login-pwd" | "login-code" | "register" | "forgot";

export const AuthPage = () => {
  const navigate = useNavigate();
  const [mode, setMode] = useState<AuthMode>("login-pwd");

  const [phone, setPhone] = useState("");
  const [password, setPassword] = useState("");
  const [code, setCode] = useState("");
  const [agreed, setAgreed] = useState(false);
  const [showPwd, setShowPwd] = useState(false);
  const [showTerms, setShowTerms] = useState<string | null>(null);

  const [countdown, setCountdown] = useState(0);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    let timer: NodeJS.Timeout | null = null;
    if (countdown > 0) {
      timer = setInterval(() => setCountdown((c) => c - 1), 1000);
    }
    return () => clearInterval(timer);
  }, [countdown]);

  const handleSendCode = async () => {
    if (!phone || phone.length < 11) return showToast("请输入完整的手机号");
    setCountdown(60);
    await AuthService.sendCode(phone);
    showToast("验证码已发送，请查收");
  };

  const handleSubmit = async () => {
    if (!agreed) return showToast("请先阅读并同意条款");
    if (!phone || phone.length < 11) return showToast("请输入正确的手机号");

    setLoading(true);
    try {
      if (mode === "login-pwd") {
        if (!password) {
          setLoading(false);
          return showToast("请输入密码");
        }
        await AuthService.login(phone, password);
        navigate("/", { replace: true });
      } else if (mode === "login-code") {
        if (!code) {
          setLoading(false);
          return showToast("请输入验证码");
        }
        await AuthService.login(phone, undefined, code);
        navigate("/", { replace: true });
      } else if (mode === "register") {
        if (!code) {
          setLoading(false);
          return showToast("请输入验证码");
        }
        await AuthService.register(phone, code, password);
        navigate("/", { replace: true });
      } else if (mode === "forgot") {
        if (!code) {
          setLoading(false);
          return showToast("请输入验证码");
        }
        if (!password) {
          setLoading(false);
          return showToast("请输入新密码");
        }
        await AuthService.resetPassword(phone, code, password);
        showToast("密码重置成功，请重新登录");
        setMode("login-pwd");
      }
    } catch (err) {
      const error = err as Error;
      showToast(error.message || "操作失败");
    } finally {
      setLoading(false);
    }
  };

  // Switch mode helper
  const changeMode = (m: AuthMode) => {
    setMode(m);
    setPhone("");
    setPassword("");
    setCode("");
  };

  const isFormValid =
    phone.length === 11 &&
    (mode === "login-pwd"
      ? password.length > 0
      : mode === "login-code"
        ? code.length > 0
        : mode === "register"
          ? code.length > 0
          : code.length > 0 && password.length > 0) &&
    agreed;

  return (
    <div className="flex flex-col h-full bg-bg-color pt-safe relative overflow-hidden">
      <div className="flex-1 flex flex-col items-center justify-center -mt-20 px-8">
        <div className="flex flex-col items-center mb-12">
          <div className="w-16 h-16 bg-[#07C160] rounded-2xl flex items-center justify-center mb-4 shadow-sm">
            <MessageCircle className="w-10 h-10 text-white fill-white" />
          </div>
          <h1 className="text-2xl font-semibold text-text-main text-center">
            {mode === "login-pwd" && "密码登录"}
            {mode === "login-code" && "验证码登录"}
            {mode === "register" && "手机号注册"}
            {mode === "forgot" && "找回密码"}
          </h1>
        </div>

        <div className="flex flex-col gap-4 w-full">
          <div className="flex items-center border-b border-border-color py-3 focus-within:border-[#07C160] transition-colors group">
            <span className="text-[16px] text-text-main mr-4 font-medium">
              +86
            </span>
            <input
              type="tel"
              placeholder="请输入手机号"
              maxLength={11}
              value={phone}
              onChange={(e) => setPhone(e.target.value.replace(/\D/g, ""))}
              className="flex-1 bg-transparent text-[16px] text-text-main outline-none placeholder:text-text-sub/50"
            />
          </div>

          {(mode === "login-pwd" ||
            mode === "register" ||
            mode === "forgot") && (
            <motion.div
              initial={{ height: 0, opacity: 0 }}
              animate={{ height: "auto", opacity: 1 }}
              exit={{ height: 0, opacity: 0 }}
              className="flex items-center border-b border-border-color py-3 focus-within:border-[#07C160] transition-colors"
            >
              <input
                type={showPwd ? "text" : "password"}
                placeholder={mode === "forgot" ? "设置新密码" : "请输入密码"}
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                className="flex-1 bg-transparent text-[16px] text-text-main outline-none placeholder:text-text-sub/50"
              />
              <div
                onClick={() => setShowPwd(!showPwd)}
                className="pl-4 pr-1 text-text-sub/50 active:scale-90 transition-transform cursor-pointer"
              >
                {showPwd ? (
                  <Eye className="w-5 h-5" />
                ) : (
                  <EyeOff className="w-5 h-5" />
                )}
              </div>
            </motion.div>
          )}

          {(mode === "login-code" ||
            mode === "register" ||
            mode === "forgot") && (
            <motion.div
              initial={{ height: 0, opacity: 0 }}
              animate={{ height: "auto", opacity: 1 }}
              exit={{ height: 0, opacity: 0 }}
              className="flex items-center border-b border-border-color py-3 focus-within:border-[#07C160] transition-colors"
            >
              <input
                type="text"
                placeholder="请输入验证码"
                maxLength={6}
                value={code}
                onChange={(e) => setCode(e.target.value.replace(/\D/g, ""))}
                className="flex-1 bg-transparent text-[16px] text-text-main outline-none placeholder:text-text-sub/50"
              />
              <button
                onClick={handleSendCode}
                disabled={countdown > 0}
                className="text-[#576B95] text-[15px] pl-4 font-medium disabled:opacity-50 active:opacity-70 transition-opacity"
              >
                {countdown > 0 ? `${countdown}s后获取` : "获取验证码"}
              </button>
            </motion.div>
          )}

          <div className="mt-8 flex flex-col gap-5">
            <button
              className={cn(
                "w-full h-12 rounded-lg text-[17px] font-medium transition-all text-white active:scale-[0.98]",
                isFormValid
                  ? "bg-[#07C160] shadow-md shadow-[#07C160]/20"
                  : "bg-[#E5E5E5] dark:bg-[#2C2C2C] text-[#B2B2B2] dark:text-[#5B5B5B]",
              )}
              disabled={loading || !isFormValid}
              onClick={handleSubmit}
            >
              {loading
                ? "请稍候..."
                : mode.startsWith("login")
                  ? "同意并登录"
                  : mode === "register"
                    ? "同意并注册"
                    : "确认"}
            </button>

            <div className="flex justify-between items-center text-[14px] text-[#576B95] px-1 font-medium">
              {mode === "login-pwd" && (
                <span
                  className="cursor-pointer active:opacity-70"
                  onClick={() => changeMode("login-code")}
                >
                  用验证码登录
                </span>
              )}
              {mode === "login-code" && (
                <span
                  className="cursor-pointer active:opacity-70"
                  onClick={() => changeMode("login-pwd")}
                >
                  用密码登录
                </span>
              )}
              {(mode === "login-pwd" || mode === "login-code") && (
                <div className="flex gap-4">
                  <span
                    className="cursor-pointer active:opacity-70"
                    onClick={() => changeMode("forgot")}
                  >
                    找回密码
                  </span>
                  <span
                    className="cursor-pointer active:opacity-70"
                    onClick={() => changeMode("register")}
                  >
                    注册账号
                  </span>
                </div>
              )}
              {(mode === "register" || mode === "forgot") && (
                <span
                  className="cursor-pointer active:opacity-70"
                  onClick={() => changeMode("login-pwd")}
                >
                  返回登录
                </span>
              )}
            </div>
          </div>
        </div>
      </div>

      <div className="pb-10 px-8 flex items-start gap-2">
        <div
          className={cn(
            "w-[18px] h-[18px] mt-0.5 rounded-full border flex items-center justify-center shrink-0 cursor-pointer transition-colors",
            agreed ? "bg-[#07C160] border-[#07C160]" : "border-text-sub/40",
          )}
          onClick={() => setAgreed(!agreed)}
        >
          {agreed && <div className="w-1.5 h-1.5 bg-white rounded-full" />}
        </div>
        <p className="text-[12px] text-text-sub leading-relaxed">
          我已阅读并同意{" "}
          <span
            className="text-[#576B95] active:opacity-70 cursor-pointer"
            onClick={() => setShowTerms("《软件许可及服务协议》")}
          >
            《软件许可及服务协议》
          </span>{" "}
          和{" "}
          <span
            className="text-[#576B95] active:opacity-70 cursor-pointer"
            onClick={() => setShowTerms("《隐私保护指引》")}
          >
            《隐私保护指引》
          </span>
        </p>
      </div>

      {showTerms && (
        <div
          className="fixed inset-0 z-50 bg-black/50 flex flex-col items-center justify-center p-6 pb-20"
          onClick={() => setShowTerms(null)}
        >
          <div
            className="bg-white dark:bg-[#1C1C1E] w-full max-w-[320px] rounded-2xl flex flex-col overflow-hidden max-h-[70vh]"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="py-4 border-b border-border-color text-center font-medium text-[16px]">
              {showTerms}
            </div>
            <div className="flex-1 overflow-y-auto p-6 text-[14px] text-text-sub leading-relaxed">
              <p className="mb-4">
                这是一段模拟的协议内容。真实环境中应展示完整的法律条文。
              </p>
              <p className="mb-4">
                1. 您必须遵守本应用的使用规范，不得利用本应用从事违法活动。
              </p>
              <p className="mb-4">
                2.
                我们会收集您的部分使用数据以优化服务，但承诺保护您的隐私安全。
              </p>
              <p>3. 若您继续使用，即表示完全理解并接受所有条款。</p>
            </div>
            <div
              className="py-4 text-center text-[#576B95] font-medium active:bg-active-bg cursor-pointer border-t border-border-color"
              onClick={() => setShowTerms(null)}
            >
              知道了
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
